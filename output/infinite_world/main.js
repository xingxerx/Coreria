(() => {
  // --- Simple pseudo-random noise for terrain ---
  function hash(x, y) {
    let h = (x * 374761393 + y * 668265263) >>> 0;
    h = (h ^ (h >>> 13)) * 1274126177 >>> 0;
    return ((h ^ (h >>> 16)) >>> 0) / 0xffffffff;
  }

  function smoothNoise(x, y) {
    const xi = Math.floor(x), yi = Math.floor(y);
    const xf = x - xi, yf = y - yi;
    const c00 = hash(xi, yi);
    const c10 = hash(xi + 1, yi);
    const c01 = hash(xi, yi + 1);
    const c11 = hash(xi + 1, yi + 1);
    const u = xf * xf * (3 - 2 * xf);
    const v = yf * yf * (3 - 2 * yf);
    return (c00 * (1 - u) + c10 * u) * (1 - v) + (c01 * (1 - u) + c11 * u) * v;
  }

  // Fractional Brownian Motion for terrain height
  function fbm(x, y) {
    let value = 0;
    let amplitude = 0.6;
    let frequency = 0.02;
    for (let i = 0; i < 5; i++) {
      value += amplitude * smoothNoise(x * frequency, y * frequency);
      amplitude *= 0.5;
      frequency *= 2.0;
    }
    return value;
  }

  // --- 3D math & projection ---
  function isoProject(px, py, pz, camera) {
    // Camera-centered isometric projection (camera follows player)
    const dx = px - (camera.wx ?? 0);
    const dy = py - (camera.wy ?? 0);
    const sx = (dx - dy) * camera.scale + camera.cx;
    const sy = (dx + dy) * 0.5 * camera.scale - pz * camera.zscale + camera.cy;
    return [sx, sy];
  }

  // --- Engine state ---
  const svg = document.getElementById('scene');
  const worldG = document.getElementById('world');
  const playerG = document.getElementById('player');
  const statsEl = document.getElementById('stats');
  const minimap = document.getElementById('minimap');
  const perfEl = document.getElementById('perf');
  const mm = minimap ? minimap.getContext('2d') : null;

  // URL params / persisted settings
  const params = new URLSearchParams(location.search);
  const saved = JSON.parse(localStorage.getItem('eoe_infinite_svg') || '{}');

  // Viewbox dimensions for centering
  const vb = svg.viewBox.baseVal;
  const viewW = vb && vb.width ? vb.width : 1200;
  const viewH = vb && vb.height ? vb.height : 800;
  const viewCX = viewW * 0.5;
  const viewCY = viewH * 0.5;

  const camera = {
    cx: viewCX,
    cy: viewCY,
    scale: Number(params.get('scale') || saved.scale || 24),
    zscale: Number(params.get('zscale') || saved.zscale || 18),
    followStrength: Number(params.get('follow') || saved.follow || 8), // higher = snappier
  };

  const player = {
    x: Number(params.get('x') || saved.x || 0),
    y: Number(params.get('y') || saved.y || 0),
    z: Number(params.get('z') || saved.z || 0),
    vx: 0,
    vy: 0,
    vz: 0,
    speed: Number(params.get('speed') || saved.speed || 7),
  };

  const keys = Object.create(null);
  window.addEventListener('keydown', e => keys[e.key.toLowerCase()] = true);
  window.addEventListener('keyup',   e => keys[e.key.toLowerCase()] = false);

  // initialize camera world-center to player
  camera.wx = player.x;
  camera.wy = player.y;

  // --- World chunking (infinite) ---
  const CHUNK = Number(params.get('chunk') || saved.chunk || 16);      // tiles per side
  const TILE = Number(params.get('tile') || saved.tile || 1);          // world units per tile
  const RENDER_RADIUS = Number(params.get('radius') || saved.radius || 6); // chunks around player
  const loaded = new Map(); // key: `cx,cy` => {element, stamp}
  let frameStamp = 0;

  function tileColor(h) {
    if (h < 0.35) return '#134e4a'; // deep
    if (h < 0.4)  return '#0f766e'; // shore
    if (h < 0.5)  return '#14532d'; // grass low
    if (h < 0.65) return '#166534'; // grass mid
    if (h < 0.8)  return '#1b5e20'; // forest
    return '#374151';                // rock
  }

  function ensureChunk(cx, cy) {
    const key = `${cx},${cy}`;
    let entry = loaded.get(key);
    if (!entry) {
      const g = document.createElementNS('http://www.w3.org/2000/svg', 'g');
      g.setAttribute('data-chunk', key);
      worldG.appendChild(g);
      entry = { element: g, stamp: 0 };
      loaded.set(key, entry);
    }
    entry.stamp = frameStamp;

    // Rebuild chunk geometry if first time or occasional refresh
    if (!entry.built) {
      entry.element.textContent = '';
      const ox = cx * CHUNK * TILE;
      const oy = cy * CHUNK * TILE;
      for (let ty = 0; ty < CHUNK; ty++) {
        for (let tx = 0; tx < CHUNK; tx++) {
          const wx = ox + tx * TILE;
          const wy = oy + ty * TILE;
          const h = fbm(wx, wy);
          const [sx, sy] = isoProject(wx, wy, h * 6, camera);
          const size = 0.9 * camera.scale;
          const r = document.createElementNS('http://www.w3.org/2000/svg', 'rect');
          r.setAttribute('x', (sx - size * 0.5).toFixed(1));
          r.setAttribute('y', (sy - size * 0.3).toFixed(1));
          r.setAttribute('width', size.toFixed(1));
          r.setAttribute('height', (size * 0.6).toFixed(1));
          r.setAttribute('fill', tileColor(h));
          r.setAttribute('opacity', '0.95');
          r.setAttribute('rx', (size * 0.1).toFixed(1));
          entry.element.appendChild(r);
        }
      }
      entry.built = true;
    }
    return entry.element;
  }

  function cullOldChunks() {
    for (const [key, entry] of loaded) {
      if (entry.stamp !== frameStamp) {
        entry.element.remove();
        loaded.delete(key);
      }
    }
  }

  function updatePlayer(dt) {
    const sprint = keys['shift'] ? 2.0 : 1.0;
    const ax = (keys['d'] ? 1 : 0) - (keys['a'] ? 1 : 0);
    const ay = (keys['s'] ? 1 : 0) - (keys['w'] ? 1 : 0);
    const az = (keys['e'] ? 1 : 0) - (keys['q'] ? 1 : 0);
    const accel = 20 * sprint;

    player.vx += ax * accel * dt;
    player.vy += ay * accel * dt;
    player.vz += az * accel * dt;

    // damping
    player.vx *= 0.9; player.vy *= 0.9; player.vz *= 0.85;

    const maxSpeed = player.speed * (1 + (sprint - 1) * 0.5);
    const clamp = (v, m) => Math.max(-m, Math.min(m, v));
    player.vx = clamp(player.vx, maxSpeed);
    player.vy = clamp(player.vy, maxSpeed);
    player.vz = clamp(player.vz, maxSpeed);

    player.x += player.vx * dt;
    player.y += player.vy * dt;
    player.z = fbm(player.x, player.y) * 6 + player.vz * 0.02; // hover above terrain

    // camera follows player smoothly
    const lerp = (a,b,t)=>a+(b-a)*t;
    const t = 1 - Math.exp(-camera.followStrength * dt);
    camera.wx = lerp(camera.wx, player.x, t);
    camera.wy = lerp(camera.wy, player.y, t);
  }

  function renderPlayer() {
    playerG.textContent = '';
    const [sx, sy] = isoProject(player.x, player.y, player.z + 1.0, camera);

    const body = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
    body.setAttribute('cx', sx.toFixed(1));
    body.setAttribute('cy', sy.toFixed(1));
    body.setAttribute('r', (camera.scale * 0.35).toFixed(1));
    body.setAttribute('fill', '#93c5fd');
    body.setAttribute('stroke', '#2563eb');
    body.setAttribute('stroke-width', '2');
    playerG.appendChild(body);

    const halo = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
    halo.setAttribute('cx', sx.toFixed(1));
    halo.setAttribute('cy', sy.toFixed(1));
    halo.setAttribute('r', (camera.scale * 0.55).toFixed(1));
    halo.setAttribute('fill', 'none');
    halo.setAttribute('stroke', 'rgba(147,197,253,0.35)');
    halo.setAttribute('stroke-width', '4');
    playerG.appendChild(halo);
  }

  // persistence throttle
  let persistTimer = 0;

  // FPS tracking
  const fpsSamples = [];

  function drawMinimap(cx, cy) {
    if (!mm) return;
    const w = minimap.width, h = minimap.height;
    mm.clearRect(0,0,w,h);
    mm.fillStyle = 'rgba(15,23,42,0.85)';
    mm.fillRect(0,0,w,h);

    // draw chunk grid around player
    const cells = 9;
    const cell = Math.floor(Math.min(w, h) / cells);
    const ox = Math.floor(cx) - Math.floor(cells/2);
    const oy = Math.floor(cy) - Math.floor(cells/2);
    for (let y = 0; y < cells; y++) {
      for (let x = 0; x < cells; x++) {
        const ccx = ox + x, ccy = oy + y;
        const key = `${ccx},${ccy}`;
        mm.strokeStyle = 'rgba(203,213,225,0.25)';
        mm.strokeRect(x*cell+0.5, y*cell+0.5, cell-1, cell-1);
        if (loaded.has(key)) {
          mm.fillStyle = 'rgba(99,102,241,0.35)';
          mm.fillRect(x*cell+1, y*cell+1, cell-2, cell-2);
        }
      }
    }

    // player marker
    mm.fillStyle = '#60a5fa';
    mm.beginPath();
    mm.arc((cells/2)*cell, (cells/2)*cell, 4, 0, Math.PI*2);
    mm.fill();
  }

  let last = performance.now();
  function frame(now) {
    const dt = Math.min(0.05, (now - last) / 1000);
    last = now;

    frameStamp++;
    updatePlayer(dt);

    // Determine current chunk and load surrounding chunks
    const ccx = Math.floor(player.x / (CHUNK * TILE));
    const ccy = Math.floor(player.y / (CHUNK * TILE));
    for (let dy = -RENDER_RADIUS; dy <= RENDER_RADIUS; dy++) {
      for (let dx = -RENDER_RADIUS; dx <= RENDER_RADIUS; dx++) {
        ensureChunk(ccx + dx, ccy + dy);
      }
    }

    renderPlayer();
    cullOldChunks();

    // minimap + fps
    drawMinimap(ccx, ccy);
    const fps = 1/Math.max(dt, 1/120);
    fpsSamples.push(fps);
    if (fpsSamples.length > 30) fpsSamples.shift();
    const avg = fpsSamples.reduce((a,b)=>a+b,0)/fpsSamples.length;

    if (perfEl) perfEl.textContent = `FPS: ${avg.toFixed(1)}  chunks: ${loaded.size}`;
    if (statsEl) statsEl.textContent = `pos=(${player.x.toFixed(1)}, ${player.y.toFixed(1)}, ${player.z.toFixed(1)})`;

    // persist some state every ~0.5s
    persistTimer += dt;
    if (persistTimer > 0.5) {
      persistTimer = 0;
      localStorage.setItem('eoe_infinite_svg', JSON.stringify({
        x: player.x, y: player.y, z: player.z,
        scale: camera.scale, zscale: camera.zscale,
        chunk: CHUNK, tile: TILE, radius: RENDER_RADIUS,
        speed: player.speed
      }));
    }

    requestAnimationFrame(frame);
  }

  requestAnimationFrame(frame);
})();

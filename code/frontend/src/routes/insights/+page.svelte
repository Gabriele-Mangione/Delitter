<script lang="ts">
  // mock data for demo purposes
  type Finding = {
    id: string;
    lat: number;
    lng: number;
    weight: number;          // g
    category: string;
    material: string;
    brand?: string | null;
    createdAt: string;       // ISO date (yyyy-mm-dd)
  };

  const demo: Finding[] = [
    { id:'1', lat:47.5596, lng:7.5886, weight:24,  category:'Beverage Can',  material:'Aluminium', brand:'Smirnoff', createdAt:'2025-10-28' },
    { id:'2', lat:47.56,   lng:7.59,   weight:18,  category:'Snack Wrapper', material:'Plastic',   brand:'Doritos', createdAt:'2025-10-29' },
    { id:'3', lat:47.558,  lng:7.592,  weight:220, category:'Plastic Bottle', material:'Plastic',   brand:'Fanta',   createdAt:'2025-11-01' },
    { id:'4', lat:47.557,  lng:7.585,  weight:9,   category:'Paper Cup',     material:'Paper',     brand:'Café',    createdAt:'2025-11-01' },
    { id:'5', lat:47.561,  lng:7.586,  weight:15,  category:'Beverage Can',  material:'Aluminium', brand:'Red Bull',createdAt:'2025-11-02' },
    { id:'6', lat:47.559,  lng:7.587,  weight:2,   category:'Cigarette Butt',material:'Other',     brand:'Coca Cola',      createdAt:'2025-11-02' },
    { id:'7', lat:47.559,  lng:7.589,  weight:240, category:'Glass Bottle',   material:'Glass',     brand:'Local',   createdAt:'2025-11-03' },
    { id:'8', lat:47.559,  lng:7.589,  weight:6,   category:'Bag',            material:'Plastic',   brand:'Coca Cola',      createdAt:'2025-11-03' },
  ];

  // summary stats
  const totalItems = demo.length;
  const totalWeightG = demo.reduce((s, d) => s + (d.weight ?? 0), 0);
  const totalWeightKg = Math.round(totalWeightG) / 1000;
  const savedCO2 = +(totalWeightKg * 1.6).toFixed(2); // Mock

  function topBy<T extends keyof Finding>(key: T) {
    const map = new Map<string, number>();
    demo.forEach(d => map.set(String(d[key] ?? '—'), (map.get(String(d[key] ?? '—')) ?? 0) + 1));
    let best = '—'; let bestN = 0;
    for (const [k, v] of map) if (v > bestN) { best = k; bestN = v; }
    return { name: best, count: bestN, map };
  }
  const topCategory = topBy('category');
  const topMaterial = topBy('material');

  const topBrands = Array.from(topBy('brand').map.entries())
    .filter(([b]) => b && b !== 'null')
    .sort((a,b) => b[1]-a[1])
    .slice(0,5);

  // weekday activity bars
  const weekdayLabels = ['Sun','Mon','Tue','Wed','Thu','Fri','Sat'];
  const weekdayCountsRaw = Array(7).fill(0) as number[];
  demo.forEach(d => {
    const day = new Date(`${d.createdAt}T12:00:00Z`).getDay();
    weekdayCountsRaw[day]++;
  });
  // never show empty columns: baseline 1
  const weekdayCounts = weekdayCountsRaw.map(n => n === 0 ? 1 : n);
  const weekdayMax = Math.max(...weekdayCounts);
  const todayIdx = new Date().getDay();

  // last 8 days trendline
  const lastDays = 8;
  const dayCounts: number[] = [];
  for (let i = lastDays - 1; i >= 0; i--) {
    const dk = new Date(Date.now() - i * 24*3600*1000).toISOString().slice(0,10);
    dayCounts.push(demo.filter(d => d.createdAt === dk).length);
  }

  // sparkline path generator
  function sparklinePath(values: number[], w=320, h=96, pad=10) {
    const max = Math.max(1, ...values);
    const step = (w - pad*2) / Math.max(1, values.length - 1);
    const pts = values.map((v,i) => {
      const x = pad + i*step;
      const y = h - pad - (v/max)*(h - pad*2);
      return [x,y] as const;
    });
    return pts.map((p,i) => (i===0?'M':'L') + p[0].toFixed(1) + ' ' + p[1].toFixed(1)).join(' ');
  }

  const materialEntries = Array.from(topMaterial.map.entries()).sort((a,b)=>b[1]-a[1]);
  const materialTotal = materialEntries.reduce((s, [,n]) => s+n, 0);
  type Seg = { label: string; percent: number; colorVar: string; };
  const paletteVars = ['--p','--a','--su','--in','--se','--wa','--er'];
  const donutSegs: Seg[] = materialEntries.map(([label, n], i) => ({
    label, percent: (n/materialTotal)*100, colorVar: paletteVars[i % paletteVars.length]
  }));
  function conicStyle(segs: Seg[]) {
    let start = 0, parts: string[] = [];
    for (const s of segs) {
      const end = start + s.percent;
      parts.push(`oklch(from hsl(var(${s.colorVar})) l c h) ${start}% ${end}%`);
      start = end;
    }
    return `conic-gradient(${parts.join(',')})`;
  }

  // ---------- GitHub-style contribution heatmap (last 12 weeks) ----------
  const WEEKS = 12;
  const MS_DAY = 24 * 3600 * 1000;

  const countsByDate = new Map<string, number>();
  for (const d of demo) {
    countsByDate.set(d.createdAt, (countsByDate.get(d.createdAt) ?? 0) + 1);
  }

  function iso(d: Date) { return d.toISOString().slice(0,10); }

  // start on the previous Sunday to align rows
  const today = new Date();
  const start = new Date(today);
  start.setHours(12,0,0,0);
  const offsetToSunday = start.getDay(); // 0..6 (Sun=0)
  start.setTime(start.getTime() - (offsetToSunday + (WEEKS-1)*7) * MS_DAY);

  type Cell = { date: string; count: number; level: number; };
  const heatmap: Cell[][] = []; // [week][dayOfWeek]
  let globalMax = 0;

  for (let w = 0; w < WEEKS; w++) {
    const col: Cell[] = [];
    for (let d = 0; d < 7; d++) {
      const cur = new Date(start.getTime() + (w*7 + d) * MS_DAY);
      const key = iso(cur);
      const count = countsByDate.get(key) ?? 0;
      globalMax = Math.max(globalMax, count);
      col.push({ date: key, count, level: 0 });
    }
    heatmap.push(col);
  }

  // quantize into 5 levels (0..4)
  const q1 = Math.max(1, Math.ceil(globalMax * 0.25));
  const q2 = Math.max(2, Math.ceil(globalMax * 0.50));
  const q3 = Math.max(3, Math.ceil(globalMax * 0.75));
  for (const week of heatmap) {
    for (const c of week) {
      if (c.count === 0) c.level = 0;
      else if (c.count <= q1) c.level = 1;
      else if (c.count <= q2) c.level = 2;
      else if (c.count <= q3) c.level = 3;
      else c.level = 4;
    }
  }

  // month labels above columns
  const monthLabels: string[] = [];
  for (let w = 0; w < WEEKS; w++) {
    const firstOfCol = heatmap[w][0].date; // Sunday of that week
    const m = new Date(firstOfCol + 'T12:00:00Z');
    monthLabels.push(m.getDate() <= 7 ? m.toLocaleString(undefined, { month: 'short' }) : '');
  }

  const levelToClass = (lvl: number) => {
    switch (lvl) {
      case 0: return 'bg-base-300/30';
      case 1: return 'bg-accent/25';
      case 2: return 'bg-accent/45';
      case 3: return 'bg-accent/65';
      default: return 'bg-accent';
    }
  };
</script>

<svelte:head><title>Insights • Delitter</title></svelte:head>

<div class="space-y-6">
  <div class="flex items-center justify-between gap-3">
    <h1 class="text-2xl md:text-3xl font-semibold">Insights</h1>
    <div class="join">
      <button class="btn btn-sm join-item btn-ghost">7d</button>
      <button class="btn btn-sm join-item btn-ghost btn-active">30d</button>
      <button class="btn btn-sm join-item btn-ghost">All</button>
    </div>
  </div>

  <!-- stats -->
  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
    <div class="card bg-base-100 shadow-sm"><div class="card-body">
      <div class="flex items-center justify-between">
        <span class="text-base-content/70">Items collected</span>
        <div class="badge">{topCategory.name}</div>
      </div>
      <div class="text-3xl font-bold">{totalItems}</div>
      <div class="text-xs text-base-content/60">Top category • {topCategory.count}</div>
    </div></div>

    <div class="card bg-base-100 shadow-sm"><div class="card-body">
      <div class="flex items-center justify-between">
        <span class="text-base-content/70">Total weight</span>
        <div class="badge badge-success badge-outline">{totalWeightG} g</div>
      </div>
      <div class="text-3xl font-bold">{totalWeightKg.toFixed(2)} kg</div>
      <div class="text-xs text-base-content/60">Est. CO₂ saved ~ {savedCO2} kg</div>
    </div></div>

    <div class="card bg-base-100 shadow-sm"><div class="card-body">
      <div class="text-base-content/70">Top material</div>
      <div class="text-3xl font-bold">{topMaterial.name}</div>
      <div class="text-xs text-base-content/60">{topMaterial.count} findings</div>
    </div></div>

    <div class="card bg-base-100 shadow-sm"><div class="card-body">
      <div class="text-base-content/70">Active volunteers</div>
      <div class="text-3xl font-bold">12</div>
      <div class="text-xs text-base-content/60">Mock value (coming soon)</div>
    </div></div>
  </div>

  <!-- charts -->
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
    <!-- sparkline -->
    <div class="card bg-base-100 shadow-sm lg:col-span-2">
      <div class="card-body">
        <div class="flex items-center justify-between">
          <h2 class="card-title">Daily trend</h2>
          <span class="text-xs text-base-content/60">last {dayCounts.length} days</span>
        </div>
        <div class="mt-2">
          <svg viewBox="0 0 320 96" class="w-full h-24">
            <defs>
              <linearGradient id="g1" x1="0" x2="0" y1="0" y2="1">
                <stop offset="0%"  stop-color="oklch(from hsl(var(--a)) 0.9 0.15 h)" stop-opacity="0.6"></stop>
                <stop offset="100%" stop-color="oklch(from hsl(var(--a)) 0.9 0.15 h)" stop-opacity="0.02"></stop>
              </linearGradient>
            </defs>
            <path d="{sparklinePath(dayCounts, 320, 96, 10)} L 310 96 L 10 96 Z" fill="url(#g1)"></path>
            <path d="{sparklinePath(dayCounts, 320, 96, 10)}" fill="none" stroke="oklch(from hsl(var(--a)) 0.9 0.15 h)" stroke-width="2.5" stroke-linecap="round"></path>
          </svg>
        </div>
        <div class="flex gap-3 text-xs text-base-content/60">
          <div class="badge badge-outline">Peak: {Math.max(...dayCounts)}</div>
          <div class="badge badge-outline">Avg: {(dayCounts.reduce((s,n)=>s+n,0)/dayCounts.length).toFixed(1)}</div>
        </div>
      </div>
    </div>

    <!--  -->
    <div class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <h2 class="card-title">Materials</h2>
        <div class="flex items-center gap-6">
          <div
            class="w-28 h-28 rounded-full border border-base-300"
            style="background: {conicStyle(donutSegs)}"
            title="Material breakdown (mock)"
          ></div>
          <div class="grow space-y-2">
            {#each donutSegs as s}
              <div class="flex items-center justify-between text-sm">
                <div class="flex items-center gap-2">
                  <span
                    class="w-3 h-3 rounded-full"
                    style="background: oklch(from hsl(var({s.colorVar})) l c h)"
                  ></span>
                  <span>{s.label}</span>
                </div>
                <span class="text-base-content/70">{s.percent.toFixed(0)}%</span>
              </div>
            {/each}
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- contribution heatmap (github styl) -->
  <div class="card bg-base-100 shadow-sm">
    <div class="card-body">
      <div class="flex items-center justify-between">
        <h2 class="card-title">Activity heatmap</h2>
        <span class="text-xs text-base-content/60">last {WEEKS} weeks</span>
      </div>

      <!-- month labels -->
      <div class="overflow-x-auto">
        <div class="pl-8 flex gap-1 mb-1">
          {#each monthLabels as label}
            <div class="w-3 text-[10px] text-base-content/50">{label}</div>
          {/each}
        </div>

        <div class="flex items-start gap-2">
          <!-- day labels -->
          <div class="flex flex-col justify-between text-[10px] text-base-content/60 h-[6.5rem] py-[2px]">
            <span>Mon</span>
            <span>Wed</span>
            <span>Fri</span>
          </div>

          <!-- weeks (columns) -->
          <div class="flex gap-1">
            {#each heatmap as week}
              <div class="flex flex-col gap-1">
                {#each week as cell, di}
                  <!-- GitHub shows Sun..Sat; we’ll show all 7 -->
                  <div
                    class={`w-3 h-3 rounded-sm ${levelToClass(cell.level)}`}
                    title={`${cell.date}: ${cell.count} finding${cell.count===1?'':'s'}`}
                  ></div>
                {/each}
              </div>
            {/each}
          </div>
        </div>
      </div>

      <!-- key -->
      <div class="mt-3 flex items-center gap-2 text-xs text-base-content/60">
        <span>Less</span>
        <div class="flex gap-1">
          <span class="w-3 h-3 rounded-sm bg-base-300/30"></span>
          <span class="w-3 h-3 rounded-sm bg-accent/25"></span>
          <span class="w-3 h-3 rounded-sm bg-accent/45"></span>
          <span class="w-3 h-3 rounded-sm bg-accent/65"></span>
          <span class="w-3 h-3 rounded-sm bg-accent"></span>
        </div>
        <span>More</span>
      </div>
    </div>
  </div>

  <!-- weekday bars + brands -->
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
    <!-- weekday activity (bars) -->
    <div class="card bg-base-100 shadow-sm lg:col-span-2">
      <div class="card-body">
        <h2 class="card-title">Activity by weekday</h2>
        <div class="mt-3 grid grid-cols-7 gap-3 items-end h-44">
          {#each weekdayCounts as n, i}
            <div class="flex flex-col items-center gap-2">
              <div
                class={`w-full rounded-t-md transition-all ${i===todayIdx ? 'bg-accent' : 'bg-accent/70'}`}
                style="height: {Math.max(8, (n/Math.max(1, weekdayMax)) * 100)}%"
                title={`${weekdayLabels[i]}: ${n}`}
              ></div>
              <div class="flex flex-col items-center leading-tight">
                <span class="text-xs text-base-content/60">{weekdayLabels[i]}</span>
                <span class="badge badge-ghost badge-xs">{n}</span>
              </div>
            </div>
          {/each}
        </div>
        <div class="mt-2 text-xs text-base-content/60">*Empty days are shown with a subtle baseline (1) so the chart stays readable.</div>
      </div>
    </div>

    <!-- brand leaderboard -->
    <div class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <h2 class="card-title">Top brands</h2>
        <ul class="mt-2 space-y-2">
          {#if topBrands.length === 0}
            <li class="text-sm text-base-content/60">No branded litter yet.</li>
          {:else}
            {#each topBrands as [brand, cnt], idx}
              <li class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                  <div class="badge badge-outline">{idx + 1}</div>
                  <span class="font-medium">{brand}</span>
                </div>
                <span class="text-base-content/70">{cnt}</span>
              </li>
            {/each}
          {/if}
        </ul>

        <div class="divider my-3"></div>
        <div class="text-xs text-base-content/60">
          *All values are demo placeholders. Real analytics coming soon.
        </div>
      </div>
    </div>
  </div>

  <!--  -->
  <div class="card bg-base-100 shadow-sm">
    <div class="card-body items-center md:items-start md:flex-row md:justify-between gap-4">
      <div>
        <h3 class="font-semibold text-lg">Share your impact</h3>
        <p class="text-sm text-base-content/60">Export a snapshot of these insights for your team or social media.</p>
      </div>
      <div class="join">
        <button class="btn btn-ghost join-item">Export PNG</button>
        <button class="btn btn-ghost join-item">Export CSV</button>
        <button class="btn btn-primary join-item">Share</button>
      </div>
    </div>
  </div>
</div>

<style>
  .card { border: 1px solid hsl(var(--b2) / .2); }
</style>

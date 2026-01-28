<script lang="ts">
  // No backend for this one, static reference
</script>

<main class="container">
  <div class="content">
    <div class="header">
      <div class="icon-wrapper"><i class="nf-md-clock_outline"></i></div>
      <h1>Cron Reference</h1>
      <p class="subtitle">Common cron expressions and syntax guide</p>
    </div>

    <div class="grid">
      <!-- Reference Card -->
      <div class="card wide">
        <h2><i class="nf-md-format_list_bulleted"></i> Syntax Guide</h2>
        <div class="syntax-viz">
          <div class="field">┌───────────── minute (0 - 59)</div>
          <div class="field">│ ┌───────────── hour (0 - 23)</div>
          <div class="field">│ │ ┌───────────── day of month (1 - 31)</div>
          <div class="field">│ │ │ ┌───────────── month (1 - 12)</div>
          <div class="field">│ │ │ │ ┌───────────── day of week (0 - 6)</div>
          <div class="expression">* * * * *</div>
        </div>
      </div>

      <!-- Common Examples -->
      <div class="card">
        <h2><i class="nf-md-star"></i> Common Patterns</h2>
        <div class="examples">
          <div class="ex"><span>* * * * *</span> Every minute</div>
          <div class="ex"><span>0 * * * *</span> Hourly</div>
          <div class="ex"><span>0 0 * * *</span> Daily @ Midnight</div>
          <div class="ex"><span>0 0 * * 0</span> Weekly (Sun)</div>
          <div class="ex"><span>0 0 1 * *</span> Monthly</div>
          <div class="ex"><span>*/5 * * * *</span> Every 5 mins</div>
        </div>
      </div>
      
       <!-- Special Strings -->
      <div class="card">
        <h2><i class="nf-md-flash"></i> Shortcuts</h2>
        <div class="examples">
          <div class="ex"><span>@yearly</span> Once a year</div>
          <div class="ex"><span>@monthly</span> Once a month</div>
          <div class="ex"><span>@weekly</span> Once a week</div>
          <div class="ex"><span>@daily</span> Once a day</div>
          <div class="ex"><span>@hourly</span> Once an hour</div>
          <div class="ex"><span>@reboot</span> At startup</div>
        </div>
      </div>
    </div>
  </div>
</main>

<style>
  .container { min-height:100vh; padding:3rem 2rem; background:linear-gradient(135deg, #1e1e2e 0%, #181825 100%); }
  .content { max-width:1000px; margin:0 auto; }
  .header { text-align:center; margin-bottom:3rem; }
  .icon-wrapper { display:inline-flex; width:80px; height:80px; background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); border-radius:18px; align-items:center; justify-content:center; margin-bottom:1.5rem; box-shadow:0 8px 24px rgba(203,166,247,0.4); animation:float 3s ease-in-out infinite; font-size:3rem; color:#1e1e2e; }
  h1 { background:linear-gradient(135deg, #cba6f7 0%, #b4befe 100%); -webkit-background-clip:text; -webkit-text-fill-color:transparent; font-size:2.5rem; margin-bottom:0.5rem; font-weight:bold; }
  .subtitle { color:#a6adc8; font-size:1.1rem; }
  
  .grid { display:grid; grid-template-columns:1fr 1fr; gap:1.5rem; }
  .card { background:rgba(24,24,37,0.7); backdrop-filter:blur(10px); padding:2rem; border-radius:20px; border:1px solid rgba(203,166,247,0.15); box-shadow:0 8px 32px rgba(0,0,0,0.3); animation:slideUp 0.5s ease-out; }
  .wide { grid-column: span 2; }
  
  h2 { color:#cba6f7; margin-bottom:1.5rem; display:flex; align-items:center; gap:0.5rem; font-size:1.4rem; }
  
  .syntax-viz { font-family:'Consolas',monospace; color:#cdd6f4; background:#11111b; padding:1.5rem; border-radius:12px; border:2px solid #313244; }
  .field { margin-bottom:0.5rem; color:#a6adc8; }
  .expression { font-size:2rem; font-weight:bold; color:#cba6f7; margin-top:1rem; letter-spacing:0.5rem; }
  
  .examples { display:flex; flex-direction:column; gap:1rem; }
  .ex { display:flex; justify-content:space-between; align-items:center; border-bottom:1px solid #313244; padding-bottom:0.5rem; color:#cdd6f4; }
  .ex span { font-family:'Consolas',monospace; color:#a6e3a1; font-weight:bold; background:rgba(166,227,161,0.1); padding:0.2rem 0.5rem; border-radius:6px; }
  
  @keyframes float { 0%, 100% { transform:translateY(0); } 50% { transform:translateY(-10px); } }
  @keyframes slideUp { from { opacity:0; transform:translateY(20px); } to { opacity:1; transform:translateY(0); } }
</style>

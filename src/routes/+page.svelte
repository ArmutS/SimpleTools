<script lang="ts">
  // Burası şimdilik boş
</script>

<main class="launcher-box">
  
  <div class="search-container">
    <span class="icon">🔎</span>
    <input type="text" placeholder="Search..." autoFocus />
  </div>

  <div class="results-container">
    <p>Sonuçlar buraya gelecek...</p>
  </div>

</main>

<style>
  /* 1. KURAL: html etiketini UNUTMA! */
  /* html ve body ekranı tamamen kaplamalı ki içindekileri ortalayabilelim */
  :global(html),
  :global(body) {
    margin: 0;
    padding: 0;
    width: 100%;
    height: 100%;
    /* Arka plan tamamen şeffaf */
    background-color: transparent !important; 
    /* İçeriği (Launcher'ı) tam ortaya hizala */
    display: flex;
    justify-content: center;
    align-items: center; 
    /* Font */
    font-family: 'Segoe UI', sans-serif;
    overflow: hidden;
  }

  /* 2. KURAL: Launcher Kutusu */
  .launcher-box {
    display: flex;
    flex-direction: column;
    
    /* Genişlik %100 olsun ama kenarlardan 20px boşluk kalsın (Gölge kesilmesin diye) */
    width: calc(100% - 40px); 
    height: calc(100% - 40px);
    
    /* Catppuccin Base Rengi (Kırmızı yerine bunu kullan) */
    background-color: #1e1e2e; 
    
    border-radius: 16px; /* Köşeleri yuvarlat */
    overflow: hidden;    /* İçerik taşmasın */

    /* CSS Gölgesi (Rust'ta kapattığımız için burası şart) */
    box-shadow: 
      0 0 0 1px rgba(255, 255, 255, 0.1),  /* İnce kenarlık */
      0 20px 50px rgba(0, 0, 0, 0.6);      /* Derinlik gölgesi */
  }

  /* 3. Arama Alanı */
  .search-container {
    height: 60px;
    display: flex;
    align-items: center;
    padding: 0 20px;
    background-color: #1e1e2e;
    border-bottom: 1px solid #313244; /* Altına ince çizgi */
  }

  .icon {
    font-size: 1.2rem;
    margin-right: 15px;
    opacity: 0.7;
  }

  input {
    width: 100%;
    height: 100%;
    background: transparent;
    border: none;
    outline: none;
    font-size: 1.5rem;
    color: #cdd6f4; /* Yazı rengi */
  }

  /* 4. Sonuç Alanı */
  .results-container {
    flex-grow: 1; /* Kalan boşluğu doldur */
    background-color: #1e1e2e;
    padding: 20px;
    color: #a6adc8;
  }
</style>

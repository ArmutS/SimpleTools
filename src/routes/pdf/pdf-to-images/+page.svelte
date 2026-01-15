<script lang="ts">
  let toolName = "PDF to Images";
  let description =
    "PDF'in her sayfasını yüksek kaliteli JPG veya PNG olarak dışarı aktarır.";
</script>

<main class="main">
  <div class="container">
    <h1>{toolName}</h1>
    <p class="description">{description}</p>

    <div class="placeholder">
      <i class="nf-md-tools" style="font-size: 4rem; opacity: 0.5;"></i>
      <h2>🚧 Geliştirme Aşamasında</h2>
      <p>Bu araç yakında kullanıma sunulacaktır.</p>
    </div>
  </div>
</main>

<style>
  .container {
    padding: 2rem;
    max-width: 900px;
    margin: 0 auto;
  }

  h1 {
    color: var(--accent);
    margin-bottom: 1rem;
  }

  .description {
    color: var(--text-secondary);
    margin-bottom: 3rem;
    font-size: 1.1rem;
    line-height: 1.6;
  }

  .placeholder {
    background: var(--bg-secondary);
    padding: 4rem 2rem;
    border-radius: 12px;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1.5rem;
  }

  .placeholder h2 {
    color: var(--accent);
    font-size: 1.8rem;
  }

  .placeholder p {
    color: var(--text-secondary);
    font-size: 1.1rem;
  }
</style>

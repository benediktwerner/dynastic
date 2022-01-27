<script lang="ts">
  let activePerson = 0;
</script>

<div class="outer-grid">
  <header>
    <span class="logo">Dynastic</span>
  </header>

  <div class="content-grid">
    <main class="card">
      <h1>Hello Bene!</h1>
    </main>

    <aside class="card">
      <h2>Albert Einstein</h2>
    </aside>
  </div>
</div>

<style lang="scss">
  :global {
    @import './global.scss';
  }

  .outer-grid {
    display: grid;
    height: 100%;
    grid-template-areas: 'header' 'main';
    grid-template-columns: auto;
    grid-template-rows: 2.9em auto;
  }

  header {
    grid-area: header;
    background-color: hsla(0, 100%, 100%, 0.09);
    box-shadow: 0px 4px 5px 0px hsla(0, 0%, 0%, 0.14),
      0px 1px 10px 0px hsla(0, 0%, 0%, 0.12),
      0px 2px 4px -1px hsla(0, 0%, 0%, 0.2);

    padding: 0.5em;

    .logo {
      font-family: 'Permanent Marker';
      font-size: 1.7em;
      color: #f9103c;
    }
  }

  .content-grid {
    display: grid;
    gap: 0.5em;
    padding: 0.5em;
    // height: 100%;
    grid-template-areas: 'main side';
    grid-template-columns: auto minmax(200px, 400px);
    grid-template-rows: auto;
    grid-area: main;

    > .card {
      height: 100%;
    }

    main {
      grid-area: main;
    }

    aside {
      grid-area: side;
    }
  }
</style>

<script>
    import Header from "../components/header.svelte";
    import Footer from "../components/footer.svelte";
</script>

<div class="page-container">
  <Header />

  <main>
        <div class = container>
            <p>lorem ipsum</p>
        </div>

        <div class = "container">
            <p>very innovative features go here</p>
        </div>

        <div class = "container">
            <p>20$ figgin dollas</p>
        </div>
  </main>

  <Footer />
</div>

<style>

    .page-container {
        display: flex;
        flex-direction: column;
        min-height: 100vh; /* Set the container's minimum height to 100% of the viewport height */
    }

    main {
        flex: 1; /* This is the key: it makes the main area grow to fill any available space */
    }


    .container{
        border-radius: 10px;
        border: 5px solid 1D0003;
        margin: 40px;
        background-color: #1C1D20;
        padding: 10px;
        justify-self: stretch;
        box-shadow: 0 2px 4px #1D0003 rgba(0,0,0,1);
    }

    p {
		color: #33FF89;
        justify-self: center;
	}
    
</style>


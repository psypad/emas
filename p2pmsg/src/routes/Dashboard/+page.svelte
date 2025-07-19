<script>
    import DashboardHeader from "../../components/Dashboard-Header.svelte";
    import Footer from "../../components/footer.svelte";
</script>


<div class = "page-container">
    <DashboardHeader />

    <main>

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

    .space{
        padding: 10px;
        justify-self: start;
    }
    .padlef{
        padding: 10px;
        justify-self: start;
    }
</style>


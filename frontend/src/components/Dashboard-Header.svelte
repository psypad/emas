<div class="head">
  <div class="main-head">
    <h1>Dashboard</h1>
  </div>

</div>


<style>
    
    * {
    box-sizing: border-box; /* Ensures padding doesn't add to width/height */
    }

    .head {
        background-color: rgb(5, 9, 7); /* Shared background for header sections */
        width: 100%; /* Full end-to-end width */
        font-family: "Ubuntu Mono", monospace; /* Assuming this from earlier; adjust if needed */
    }

    .main-head {
        display: flex;
        align-items: center;
        justify-content: space-between; /* Heading left, button right with space */
        padding: 10px 2rem; /* Top/bottom 10px, left/right 2rem for spacing from corners */
        min-height: 60px;
        width: 100%; /* Full width */
        border-bottom: 1px solid #33FF89; /* Subtle separator, from your original */
    }

    .main-head h1 {
        margin: 0; /* Remove default margin */
        white-space: nowrap; /* Prevents wrapping (optional) */
        overflow: hidden; /* Hide overflow if too long */
        text-overflow: ellipsis; /* Ellipsis if text is clipped */
        color: #fff;
    }

    /* Responsive media query: Stack nav on small screens (new addition) */
    @media (max-width: 768px) {
        .main-head {
            padding: 10px 1rem; /* Reduce padding for mobile */
        }
    }
</style>
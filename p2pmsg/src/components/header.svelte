<div class="head">
  <div class="main-head">
    <h1>Welcome to The Proj</h1>
    <button>login</button>
  </div>

  <div>
    <nav class="subhead-2">
        <ol>
        <li><a href="/login">login</a></li>
        <li><a href="/register">Register</a></li>
        <li><a href="/Dashboard">Dashboard</a></li>
        <li><a href="#pricing">Pricing</a></li>
        <li><a href="#Features">Features</a></li>
        </ol>
    </nav>
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

    .main-head button {
        margin-left: 2rem; /* Space between h1 and button */
        padding: 0.5rem 1rem; /* Internal padding for button size */
        font-size: 1rem;
        background: #222; /* Dark background to match theme */
        color: #fff;
        border: none;
        border-radius: 4px;
        cursor: pointer; /* Better UX for interactivity */
        transition: background 0.2s; /* Smooth hover effect (new addition) */
    }

    .main-head button:hover {
        background: #333; /* Hover state for feedback (new addition) */
    }

    .subhead-2 {
        display: flex; /* Flex on nav container for overall alignment */
        align-items: center; /* Vertically center the list */
        padding: 10px 2rem; /* Consistent padding for spacing from corners */
        border-bottom: 1px solid #33FF89; /* Subtle separator */
        background-color: rgb(5, 9, 7); /* Match main-head background */
    }

    .subhead-2 ol {
        display: flex;
        gap: 1.5rem; /* Space between nav items */
        list-style: none; /* Remove default bullets (new addition) */
        margin: 0;
        padding: 0;
        justify-content: flex-start; /* Left-aligned (corrected from 'start') */
    }

    .subhead-2 a {
        color: #33FF89; /* Your green link color */
        text-decoration: none; /* Remove underline for clean look (new addition) */
        transition: color 0.2s; /* Smooth hover (new addition) */
    }

    .subhead-2 a:hover {
        color: #66FFC2; /* Lighter green on hover for feedback (new addition) */
    }

    /* Responsive media query: Stack nav on small screens (new addition) */
    @media (max-width: 768px) {
        .main-head {
            padding: 10px 1rem; /* Reduce padding for mobile */
        }
        
        .subhead-2 {
            padding: 10px 1rem;
        }
        
        .subhead-2 ol {
            flex-direction: column; /* Stack vertically on mobile */
            gap: 0.5rem;
        }
    }
</style>
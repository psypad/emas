<script>
  // You can add any props or logic here if needed
</script>

<aside>
  <h3>Dashboard Navigation</h3>
  <ul>
    <li><a href="/dashboard">Overview</a></li>
    <li><a href="/dashboard/settings">Settings</a></li>
    <li><a href="/dashboard/profile">Profile</a></li>
    </ul>
</aside>

<style>
  aside {
    width: 200px; /* Adjust width as desired */
    background-color: #f0f0f0;
    padding: 20px;
    box-shadow: 2px 0 5px rgba(0,0,0,0.1);
    height: 100vh; /* Make sidebar full height */
    position: fixed; /* Keep sidebar in place when scrolling */
    top: 0;
    left: 0;
  }

  ul {
    list-style: none;
    padding: 0;
  }

  li {
    margin-bottom: 10px;
  }

  a {
    text-decoration: none;
    color: #333;
    font-weight: bold;
  }

  a:hover {
    color: #007bff;
  }
</style>
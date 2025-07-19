<script>

</script>

<div class = "container">

</div>

<form action="" method="get" class = "cform">
  <div class="inputBox">
      <textarea name="text" id="text" required></textarea>
  </div>
  <div class="send">
    <input type="submit" value="send" />
  </div>
</form>

<style>
    /* Basic Reset */
    * {
      margin: 0;
      padding: 0;
      box-sizing: border-box;
    }

    .container {
      background-color: #090C09;
      height: 83vh;
      display: flex;
      flex-direction: column;
      justify-content: flex-end; /* Push form to bottom */
      font-family: Arial, sans-serif;
    }

    .cform {
      display: flex;
      padding: 10px;
      background-color: #444;
      height: 10vh;
    }

    .inputBox {
      flex: 1;
      margin-right: 10px;
    }

    textarea {
      width: 100%;
      height: 100px;
      resize: none;
      padding: 10px;
      font-size: 16px;
    }

    .send input[type="submit"] {
      height: 100px;
      padding: 0 20px;
      font-size: 16px;
    }
</style>
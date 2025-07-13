<script>
    function greet() {
        return `Hello, ${name}!`;
    }
</script>


<fieldset class = "field">
  <legend>Register:</legend>
    <form action="" method="get" class="form-example">
        <div class="form-example">
            <label for="email">Enter your email: </label>
            <input type="email" name="email" id="email" required />
        </div>
        <div class="form-example">
            <label for="password">Enter your password: </label>
            <input type="text" name="password" id="password" required />
        </div>
        <div class="form-example">
            <label for="password">Enter your password again: </label>
            <input type="text" name="password" id="password" required />
        </div>
        <div class = "form-example" >
            <input type="checkbox" id="Terms" name="Terms" checked />
            <label for="Terms">Agree to the <a href = " ">Terms and Conditions</a></label>
        </div>

        <div class="submit">
            <input type="submit" value="Register" />
        </div>
    </form>

</fieldset>

<style>
    .form-example{
        padding: 10px;
    }

    .submit{
        padding: 15px;
        justify-self: center;
        
    }

    .field{
        justify-self: anchor-center;
    }
    

</style>
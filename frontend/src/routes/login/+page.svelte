<script lang='ts'>
    import { goto } from '$app/navigation';
    
    let email: string = '';
    let password: string = '';
    let isLoading: boolean = false;
    let error: string = '';

    interface LoginResponse {
        token: string;
        message?: string;
    }

    interface ErrorResponse {
        message: string;
    }

    async function handleLogin(): Promise<void> {
        if (!email || !password) {
        error = 'Please fill in all fields';
        return;
        }

        isLoading = true;
        error = '';

        try {
        // TODO: Replace with actual API call
        const response = await fetch('http://localhost:3000/api/auth/login', {
            method: 'POST',
            headers: {
            'Content-Type': 'application/json'
            },
            body: JSON.stringify({ email, password })
        });


        if (response.ok) {
            const data: LoginResponse = await response.json();
            // Store token (you might want to use a store for this)
            localStorage.setItem('token', data.token);
            goto('/Dashboard');
        } else {
            const errorData: ErrorResponse = await response.json();
            error = errorData.message || 'Login failed';
        }
        } catch (err) {
        error = 'Network error. Please try again.';
        } finally {
        isLoading = false;
        }
    }

    function handleKeydown(event: KeyboardEvent): void {
        if (event.key === 'Enter') {
        handleLogin();
        }
    }
</script>


<fieldset class = "field">
  <legend>Login:</legend>
    <form action="" method="get" class="form-example">
        <div class="form-example">
            <label for="email">Enter your email: </label>
            <input bind:value={email} type="email" name="email" id="email" required />
        </div>

        <div class="form-example">
            <label for="password">Enter your password: </label>
            <input  bind:value={password} type="text" name="password" id="password" required />
        </div>

        <div class = "form-example">
            <input type="checkbox" id="remember" name="remember" checked />
            <label for="remember">Remember me</label>
        </div>

        <div class="submit">
            <input type="submit" on:click={handleLogin} value="Login" />
        </div>
    </form>

    {#if error}
        <p style="color: red;">{error}</p>
    {/if}


</fieldset>




<style>
    .form-example{
        padding: 10px;
    }
    
    .field{
        justify-self: anchor-center;
    }

    .submit{
        padding: 15px;
        justify-self: center;
        
    }

</style>
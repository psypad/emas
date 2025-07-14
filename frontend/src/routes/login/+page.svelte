<script lang="ts">
  import { goto } from '$app/navigation';

  let email: string = '';
  let password: string = '';
  let remember: boolean = false;
  let isLoading: boolean = false;
  let error: string = '';
  let success: string = '';

  interface LoginResponse {
    token: string;
    message: string;
  }

  interface ErrorResponse {
    error: string;
  }

  async function handleLogin(event: Event): Promise<void> {
    event.preventDefault();
    if (!email.includes('@') || password.length < 8) {
      error = 'Please enter a valid email and a password with at least 8 characters.';
      return;
    }

    isLoading = true;
    error = '';
    success = '';

    try {
      const response = await fetch('http://localhost:3000/api/auth/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email, password }),
      });

      if (response.ok) {
        const data: LoginResponse = await response.json();
        if (remember) {
          localStorage.setItem('token', data.token);
        } else {
          sessionStorage.setItem('token', data.token);
        }
        success = data.message;
        goto('/Dashboard');
      } else {
        const errorData: ErrorResponse = await response.json();
        error = errorData.error || 'Login failed';
      }
    } catch (err) {
      error = 'Network error. Please try again.';
    } finally {
      isLoading = false;
    }
  }
</script>

<fieldset class="field">
  <legend>Login</legend>
  <form on:submit={handleLogin}>
    <div class="form-example">
      <label for="email">Email:</label>
      <input
        bind:value={email}
        type="email"
        name="email"
        id="email"
        required
        placeholder="Enter your email"
      />
    </div>
    <div class="form-example">
      <label for="password">Password:</label>
      <input
        bind:value={password}
        type="password"
        name="password"
        id="password"
        required
        placeholder="Enter your password"
      />
    </div>
    <div class="form-example">
      <input
        bind:checked={remember}
        type="checkbox"
        id="remember"
        name="remember"
      />
      <label for="remember">Remember me</label>
    </div>
    <div class="submit">
      <button type="submit" disabled={isLoading}>
        {isLoading ? 'Logging in...' : 'Login'}
      </button>
    </div>
  </form>

  {#if error}
    <p style="color: red;">{error}</p>
  {/if}
  {#if success}
    <p style="color: green;">{success}</p>
  {/if}
</fieldset>

<style>
  .field {
    max-width: 400px;
    margin: 0 auto;
    padding: 20px;
    border: 1px solid #ccc;
    border-radius: 8px;
  }
  .form-example {
    padding: 10px;
    display: flex;
    flex-direction: column;
    gap: 5px;
  }
  .submit {
    padding: 15px;
    text-align: center;
  }
  label {
    font-weight: bold;
  }
  input[type="email"],
  input[type="password"] {
    padding: 8px;
    border: 1px solid #ccc;
    border-radius: 4px;
  }
  button {
    padding: 10px;
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
  button:disabled {
    background-color: #6c757d;
    cursor: not-allowed;
  }
  button:hover:not(:disabled) {
    background-color: #0056b3;
  }
</style>
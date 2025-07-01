<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>API Meter - Track, Manage, Scale Your APIs</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0/css/all.min.css" rel="stylesheet">
    <style>
        .gradient-bg {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        }
        .card-hover {
            transition: all 0.3s ease;
        }
        .card-hover:hover {
            transform: translateY(-5px);
            box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
        }
    </style>
</head>
<body class="bg-gray-50">
    <!-- Navigation -->
    <nav class="bg-white shadow-lg sticky top-0 z-50">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="flex justify-between h-16">
                <div class="flex items-center">
                    <div class="flex-shrink-0 flex items-center">
                        <i class="fas fa-chart-line text-2xl text-indigo-600 mr-2"></i>
                        <span class="text-xl font-bold text-gray-900">API Meter</span>
                    </div>
                </div>
                <div class="flex items-center space-x-4">
                    <a href="#features" class="text-gray-700 hover:text-indigo-600 px-3 py-2 rounded-md text-sm font-medium transition-colors">Features</a>
                    <a href="#pricing" class="text-gray-700 hover:text-indigo-600 px-3 py-2 rounded-md text-sm font-medium transition-colors">Pricing</a>
                    <a href="/login" class="text-indigo-600 hover:text-indigo-500 px-3 py-2 rounded-md text-sm font-medium transition-colors">Login</a>
                    <a href="/register" class="bg-indigo-600 hover:bg-indigo-700 text-white px-4 py-2 rounded-md text-sm font-medium transition-colors">Get Started</a>
                </div>
            </div>
        </div>
    </nav>

    <!-- Hero Section -->
    <section class="gradient-bg text-white py-20">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center">
                <h1 class="text-4xl md:text-6xl font-bold mb-6">
                    Track Every API Call
                    <span class="block text-yellow-300">Scale with Confidence</span>
                </h1>
                <p class="text-xl md:text-2xl mb-8 max-w-3xl mx-auto opacity-90">
                    Powerful API metering and analytics platform. Monitor usage, manage keys, and optimize your API infrastructure with real-time insights.
                </p>
                <div class="space-x-4">
                    <a href="/register" class="bg-white text-indigo-600 px-8 py-3 rounded-lg text-lg font-semibold hover:bg-gray-100 transition-colors inline-block">
                        Start Free Trial
                    </a>
                    <a href="#demo" class="border-2 border-white text-white px-8 py-3 rounded-lg text-lg font-semibold hover:bg-white hover:text-indigo-600 transition-colors inline-block">
                        Watch Demo
                    </a>
                </div>
            </div>
        </div>
    </section>

    <!-- Features Section -->
    <section id="features" class="py-20 bg-white">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="text-center mb-16">
                <h2 class="text-3xl md:text-4xl font-bold text-gray-900 mb-4">
                    Everything You Need to Monitor Your APIs
                </h2>
                <p class="text-xl text-gray-600 max-w-2xl mx-auto">
                    Comprehensive API management tools designed for developers and businesses of all sizes.
                </p>
            </div>

            <div class="grid md:grid-cols-3 gap-8">
                <!-- Feature 1 -->
                <div class="card-hover bg-gray-50 p-8 rounded-xl text-center">
                    <div class="bg-indigo-100 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-6">
                        <i class="fas fa-key text-2xl text-indigo-600"></i>
                    </div>
                    <h3 class="text-xl font-semibold text-gray-900 mb-4">API Key Management</h3>
                    <p class="text-gray-600">
                        Create, manage, and revoke API keys with ease. Associate keys with users and set custom permissions for granular access control.
                    </p>
                </div>

                <!-- Feature 2 -->
                <div class="card-hover bg-gray-50 p-8 rounded-xl text-center">
                    <div class="bg-green-100 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-6">
                        <i class="fas fa-chart-bar text-2xl text-green-600"></i>
                    </div>
                    <h3 class="text-xl font-semibold text-gray-900 mb-4">Real-time Analytics</h3>
                    <p class="text-gray-600">
                        Track API usage in real-time with detailed metrics, request logs, and performance insights to optimize your services.
                    </p>
                </div>

                <!-- Feature 3 -->
                <div class="card-hover bg-gray-50 p-8 rounded-xl text-center">
                    <div class="bg-purple-100 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-6">
                        <i class="fas fa-shield-alt text-2xl text-purple-600"></i>
                    </div>
                    <h3 class="text-xl font-semibold text-gray-900 mb-4">Rate Limiting</h3>
                    <p class="text-gray-600">
                        Protect your APIs with intelligent rate limiting. Set custom limits per key and prevent abuse with advanced throttling.
                    </p>
                </div>

                <!-- Feature 4 -->
                <div class="card-hover bg-gray-50 p-8 rounded-xl text-center">
                    <div class="bg-yellow-100 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-6">
                        <i class="fas fa-users text-2xl text-yellow-600"></i>
                    </div>
                    <h3 class="text-xl font-semibold text-gray-900 mb-4">User Management</h3>
                    <p class="text-gray-600">
                        Associate API keys with user identities, track usage per user, and manage access levels across your organization.
                    </p>
                </div>

                <!-- Feature 5 -->
                <div class="card-hover bg-gray-50 p-8 rounded-xl text-center">
                    <div class="bg-red-100 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-6">
                        <i class="fas fa-bell text-2xl text-red-600"></i>
                    </div>
                    <h3 class="text-xl font-semibold text-gray-900 mb-4">Smart Alerts</h3>
                    <p class="text-gray-600">
                        Get notified when usage thresholds are reached, unusual patterns are detected, or when keys need attention.
                    </p>
                </div>

                <!-- Feature 6 -->
                <div class="card-hover bg-gray-50 p-8 rounded-xl text-center">
                    <div class="bg-blue-100 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-6">
                        <i class="fas fa-code text-2xl text-blue-600"></i>
                    </div>
                    <h3 class="text-xl font-semibold text-gray-900 mb-4">Developer Friendly</h3>
                    <p class="text-gray-600">
                        RESTful API, comprehensive documentation, and SDKs in multiple languages for seamless integration.
                    </p>
                </div>
            </div>
        </div>
    </section>

    <!-- Stats Section -->
    <section class="py-16 bg-indigo-600 text-white">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="grid md:grid-cols-4 gap-8 text-center">
                <div>
                    <div class="text-4xl font-bold mb-2">10M+</div>
                    <div class="text-indigo-200">API Calls Tracked</div>
                </div>
                <div>
                    <div class="text-4xl font-bold mb-2">500+</div>
                    <div class="text-indigo-200">Active Developers</div>
                </div>
                <div>
                    <div class="text-4xl font-bold mb-2">99.9%</div>
                    <div class="text-indigo-200">Uptime SLA</div>
                </div>
                <div>
                    <div class="text-4xl font-bold mb-2">24/7</div>
                    <div class="text-indigo-200">Support Available</div>
                </div>
            </div>
        </div>
    </section>

    <!-- CTA Section -->
    <section class="py-20 bg-gray-900 text-white">
        <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
            <h2 class="text-3xl md:text-4xl font-bold mb-6">
                Ready to Take Control of Your APIs?
            </h2>
            <p class="text-xl text-gray-300 mb-8">
                Join thousands of developers who trust API Meter for their monitoring needs.
            </p>
            <div class="space-x-4">
                <a href="/register" class="bg-indigo-600 hover:bg-indigo-700 text-white px-8 py-3 rounded-lg text-lg font-semibold transition-colors inline-block">
                    Get Started Free
                </a>
                <a href="/login" class="border-2 border-gray-600 text-gray-300 hover:bg-gray-800 px-8 py-3 rounded-lg text-lg font-semibold transition-colors inline-block">
                    Sign In
                </a>
            </div>
        </div>
    </section>

    <!-- Footer -->
    <footer class="bg-gray-800 text-gray-300 py-12">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="grid md:grid-cols-4 gap-8">
                <div>
                    <div class="flex items-center mb-4">
                        <i class="fas fa-chart-line text-xl text-indigo-400 mr-2"></i>
                        <span class="text-lg font-bold text-white">API Meter</span>
                    </div>
                    <p class="text-gray-400">
                        The most comprehensive API monitoring and analytics platform for modern developers.
                    </p>
                </div>
                <div>
                    <h4 class="text-white font-semibold mb-4">Product</h4>
                    <ul class="space-y-2">
                        <li><a href="#" class="hover:text-indigo-400 transition-colors">Features</a></li>
                        <li><a href="#" class="hover:text-indigo-400 transition-colors">Pricing</a></li>
                        <li><a href="#" class="hover:text-indigo-400 transition-colors">Documentation</a></li>
                        <li><a href="#" class="hover:text-indigo-400 transition-colors">API Reference</a></li>
                    </ul>
                </div>
                <div>
                    <h4 class="text-white font-semibold mb-4">Company</h4>
                    <ul class="space-y-2">
                        <li><a href="#" class="hover:text-indigo-400 transition-colors">About</a></li>
                        <li><a href="#" class="hover:text-indigo-400 transition-colors">Blog</a></li>
                        <li><a href="#" class="hover:text-indigo-400 transition-colors">Careers</a></li>
                        <li><a href="#" class="hover:text-indigo-400 transition-colors">Contact</a></li>
                    </ul>
                </div>
                <div>
                    <h4 class="text-white font-semibold mb-4">Support</h4>
                    <ul class="space-y-2">
                        <li><a href="#" class="hover:text-indigo-400 transition-colors">Help Center</a></li>
                        <li><a href="#" class="hover:text-indigo-400 transition-colors">Status Page</a></li>
                        <li><a href="#" class="hover:text-indigo-400 transition-colors">Security</a></li>
                        <li><a href="#" class="hover:text-indigo-400 transition-colors">Privacy</a></li>
                    </ul>
                </div>
            </div>
            <div class="border-t border-gray-700 mt-8 pt-8 text-center text-gray-400">
                <p>&copy; 2025 API Meter. All rights reserved.</p>
            </div>
        </div>
    </footer>

    <script>
        // Smooth scrolling for anchor links
        document.querySelectorAll('a[href^="#"]').forEach(anchor => {
            anchor.addEventListener('click', function (e) {
                e.preventDefault();
                const target = document.querySelector(this.getAttribute('href'));
                if (target) {
                    target.scrollIntoView({
                        behavior: 'smooth',
                        block: 'start'
                    });
                }
            });
        });

        // Add scroll effect to navbar
        window.addEventListener('scroll', function() {
            const nav = document.querySelector('nav');
            if (window.scrollY > 50) {
                nav.classList.add('bg-white/95', 'backdrop-blur-sm');
            } else {
                nav.classList.remove('bg-white/95', 'backdrop-blur-sm');
            }
        });
    </script>
</body>
</html>
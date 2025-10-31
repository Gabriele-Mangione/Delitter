<script>
  import "../app.css";
</script>
<div class="min-h-screen bg-base-200">
    <div class="navbar bg-base-100 shadow-lg">
        <div class="navbar-start">
            <div class="dropdown">
                <div tabindex="0" role="button" class="btn btn-ghost lg:hidden">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h8m-8 6h16" /></svg>
                </div>
                <ul tabindex="0" class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52">
                    <li><a>Home</a></li>
                </ul>
            </div>
            <a class="btn btn-ghost text-xl">daisyUI + Svelte</a>
        </div>

        <div class="navbar-center hidden lg:flex">
            <ul class="menu menu-horizontal px-1">
                <li><a>Home</a></li>
            </ul>
        </div>

        <div class="navbar-end">
            <a class="btn btn-primary">Get Started</a>
        </div>
    </div>

    <div class="hero min-h-[40vh] bg-base-200">
        <div class="hero-content text-center">
            <div class="max-w-md">
                <h1 class="text-5xl font-bold">Delitter</h1>
                <button class="btn btn-secondary">Start</button>
            </div>
        </div>
    </div>

    <div class="p-8">
        <h2 class="text-3xl font-bold mb-6 text-center">Reports</h2>
        <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3 max-w-7xl mx-auto">

            <div class="card w-full bg-base-100 shadow-xl">
                <figure><div class="h-48 w-full bg-primary/20 flex items-center justify-center">Card Image</div></figure>
                <div class="card-body">
                    <h3 class="card-title">Card Title One</h3>
                    <p>Uses the `card` and `card-body` classes for styling.</p>
                    <div class="card-actions justify-end">
                        <button class="btn btn-sm btn-info">View</button>
                    </div>
                </div>
            </div>

            <div class="card w-full bg-base-100 shadow-xl">
                <figure><div class="h-48 w-full bg-secondary/20 flex items-center justify-center">Card Image</div></figure>
                <div class="card-body">
                    <h3 class="card-title">Card Title Two</h3>
                    <p>The grid adapts seamlessly to the viewport width.</p>
                    <div class="card-actions justify-end">
                        <button class="btn btn-sm btn-info">View</button>
                    </div>
                </div>
            </div>

            <div class="card w-full bg-base-100 shadow-xl">
                <figure><div class="h-48 w-full bg-accent/20 flex items-center justify-center">Card Image</div></figure>
                <div class="card-body">
                    <h3 class="card-title">Card Title Three</h3>
                    <p>Resize your browser to see the responsiveness in action!</p>
                    <div class="card-actions justify-end">
                        <button class="btn btn-sm btn-info">View</button>
                    </div>
                </div>
            </div>

        </div>
    </div>
</div>
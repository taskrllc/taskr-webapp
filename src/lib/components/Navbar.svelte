<script module lang="ts">
    export type NavLink = {
        name: string;
        href: string;
    };
</script>

<script lang="ts">
    import { user } from "$lib/stores/auth";
    import { page } from "$app/state";
    import { asset } from "$app/paths";

    export let links: NavLink[] = [];
</script>

<div class="navbar bg-base-100 shadow-sm">
    <div class="navbar-start">
        <a class="btn btn-ghost text-xl" href={asset("/")}>taskr</a>
    </div>
    <div class="navbar-center hidden lg:flex">
        <ul class="menu menu-horizontal px-1">
            {#each links as link}
                {#if page.url.pathname === link.href}
                    <b><li><a href={link.href}>{link.name}</a></li></b>
                {:else}
                    <li><a href={link.href}>{link.name}</a></li>
                {/if}
            {/each}
        </ul>
    </div>
    <div class="navbar-end">
        {#if page.url.pathname !== asset("/login") && page.url.pathname !== asset("/register")}
            {#if $user}
                <button title="" class="btn btn-ghost btn-circle">
                    <div class="indicator">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-5 w-5"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"
                            />
                        </svg>
                        <span
                            class="badge badge-xs badge-primary indicator-item"
                        ></span>
                    </div>
                </button>
                <div class="dropdown dropdown-end">
                    <div
                        tabindex="0"
                        role="button"
                        class="btn btn-ghost btn-circle avatar"
                    >
                        <div class="w-10 rounded-full">
                            <img
                                alt="Tailwind CSS Navbar component"
                                src="https://img.daisyui.com/images/stock/photo-1534528741775-53994a69daeb.webp"
                            />
                        </div>
                    </div>
                    <ul
                        tabindex="-1"
                        class="menu menu-sm dropdown-content bg-base-100 rounded-box z-1 mt-3 w-52 p-2 shadow"
                    >
                        <li>
                            <a
                                class="justify-between"
                                href={asset("/dashboard")}
                            >
                                Profile
                                <span class="badge">New</span>
                            </a>
                        </li>
                        <li><a href={asset("/settings")}>Settings</a></li>
                    </ul>
                </div>
            {:else}
                <a class="btn btn-accent" href={asset("/login")}>Login</a>
            {/if}
        {/if}
    </div>
</div>

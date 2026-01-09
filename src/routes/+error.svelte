<script lang="ts">
    import { page } from "$app/state";
</script>

<div class="hero bg-base-200 min-h-screen">
    <div class="hero-content text-center">
        <div class="max-w-md">
            <h1 class="text-5xl font-bold">Oops!</h1>
            <p class="py-6"><b>{page.status}</b> - {page.error?.message}</p>
            <a href="/" class="btn btn-primary"
                >&LeftArrow; &MediumSpace; Home</a
            >
        </div>
    </div>
</div>

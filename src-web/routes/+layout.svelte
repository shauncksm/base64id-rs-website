<script>
    import Github from "./github.svg";
</script>

<svelte:head>
    <title>Base64id-rs</title>
    <meta
        name="description"
        content="Base64id is a rust library for encoding 64, 32 and 16 bit integers as
        base64url strings." />
</svelte:head>

<header class="p-4 flex flex-col gap-4">
    <div class="container mx-auto">
        <h2 class="text-2xl">Base64id</h2>
        <p>
            A Rust library for encoding 64, 32 and 16 bit integers as base64url
            strings.
        </p>

        <nav
            class="absolute top-0 left-0 w-full pt-4 flex justify-end gap-5 px-4">
            <a
                class="hover:text-gray-400 active:text-gray-200 flex items-center gap-1"
                href="https://github.com/shauncksm/base64id-rs">
                <img src={Github} alt="GH Logo" class="h-4" />
                <spam>GitHub</spam>
            </a>
            <a
                class="hover:text-gray-400 active:text-gray-200"
                href="https://crates.io/crates/base64id">Crates.io</a>
            <a
                class="hover:text-gray-400 active:text-gray-200"
                href="https://docs.rs/base64id/latest/base64id/">API Docs</a>
        </nav>
    </div>
</header>

<slot />

<footer
    class="sticky top-full h-12 w-full text-gray-400 flex gap-2 px-4 items-center text-sm">
    <span>Copyright &copy; 2023 Shaun Priestley</span>
    |
    <a
        href="https://github.com/shauncksm/base64id-rs-website"
        class="hover:underline">
        Website Repo
    </a>
    |
    <a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/">
        <img
            alt="Creative Commons License"
            style="border-width:0"
            src="https://i.creativecommons.org/l/by-sa/4.0/80x15.png" />
    </a>
</footer>

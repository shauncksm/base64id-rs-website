<script>
    import Base64idTool from "./base64id-tool.svelte";
</script>

<main class="h-full relative">
    <section class="p-4 relative">
        <div
            class="absolute top-0 left-0 w-full h-full -z-10 bg-gradient-to-br from-gray-700 to-gray-800" />
        <div class="container mx-auto grid grid-cols-1 lg:grid-cols-2">
            <Base64idTool />
        </div>
    </section>

    <article class="p-4">
        <div class="container mx-auto grid grid-cols-1 lg:grid-cols-2">
            <div class="flex flex-col gap-4">
                <h2 class="text-2xl border-b-2 border-gray-700 mt-4">
                    The Premise
                </h2>

                <p>
                    A common requirement for websites is the need to exchange
                    unique identifiers with clients. These identifiers could be
                    located within a URL or a JSON response.
                </p>

                <p>
                    An identifier can be represented as an integer or a string.
                    Our integer could be a 32 bit signed integer, and our string
                    could be a base64url string.
                </p>

                <div class="flex gap-4">
                    <figure class="bg-gray-900 p-4">
                        <figcaption class="text-gray-200 text-sm">
                            Integer
                        </figcaption>
                        <mono class="text-orange-400">1465731962</mono>
                    </figure>

                    <figure class="bg-gray-900 p-4">
                        <figcaption class="text-gray-200 text-sm">
                            String
                        </figcaption>
                        <mono class="text-blue-400">"V11Leg"</mono>
                    </figure>
                </div>

                <p>
                    Integers have the advantage of server side efficiency, been
                    very easily stored and manipulated in memory and within
                    databases. Strings have the advantage of density, that is
                    you can use fewer characters to represent more unique values
                    than you could with the same number of integer digits.
                </p>

                <p>
                    When sending a URL safe integer over the web however, you
                    loose space efficiency since the integer must be sent as a
                    UTF-8 string, such as with JSON or XML encoding.
                </p>

                <div class="flex gap-4">
                    <figure class="bg-gray-900 p-4">
                        <figcaption class="text-gray-200 text-sm">
                            32 Bit Integer
                        </figcaption>
                        <mono class="text-orange-400">1465731962</mono>
                        <figcaption class="text-gray-400 text-mono">
                            4 bytes
                        </figcaption>
                    </figure>

                    <figure class="bg-gray-900 p-4">
                        <figcaption class="text-gray-200 text-sm">
                            10 Character UTF-8 String
                        </figcaption>
                        <mono class="text-blue-400">"1465731962"</mono>
                        <figcaption class="text-gray-400 text-mono">
                            10 bytes
                        </figcaption>
                    </figure>
                </div>

                <p>
                    While both integers and strings look and are manipulated
                    differently from each other, for the purpose of working as a
                    unique identifier, the main thing we care about is that we
                    have a value, which contains a series of bytes that are
                    unique within some range. How those bytes are represented,
                    is largely at the discretion of the developer.
                </p>

                <div class="flex items-start gap-4 text-left">
                    <figure class="bg-gray-900 p-4">
                        <figcaption class="text-gray-200 text-sm">
                            Binary
                        </figcaption>
                        <mono class="text-red-400 flex flex-col">
                            <span>0101 0111 0101 1101</span>
                            <span>0100 1011 0111 1010</span>
                        </mono>
                    </figure>

                    <figure class="bg-gray-900 p-4">
                        <figcaption class="text-gray-200 text-sm">
                            Decimal
                        </figcaption>
                        <mono class="text-orange-400">1465731962</mono>
                    </figure>

                    <figure class="bg-gray-900 p-4">
                        <figcaption class="text-gray-200 text-sm">
                            Base64url
                        </figcaption>
                        <mono class="text-blue-400">"V11Leg"</mono>
                    </figure>
                </div>

                <h2 class="text-2xl border-b-2 border-gray-700 mt-4">
                    The Problem
                </h2>

                <p>
                    As a developer you may decide to represent your identifiers
                    as integers. This is often the most straightforward option,
                    especially from a database design perspective. The key
                    disadvantage in this context is the increased HTTP payload
                    sizes, when compared to a string based equivalent.
                </p>

                <p>
                    Alternatively you may go with base64url strings. This can
                    reduce the payload size issue while maintaining URL safety,
                    at the expense of increased server side memory and database
                    storage requirements.
                </p>

                <figure
                    class="bg-gray-900 p-4 grid grid-cols-[auto_1fr_1fr] text-left gap-3 leading-none">
                    <h4 class="col-start-2 col-end-2">Integer</h4>
                    <h4 class="">Base64url String</h4>

                    <h4 class="text-green-400">Pro</h4>
                    <p class="text-green-400">Smaller and most compatible</p>
                    <p class="text-green-400">Smaller url safe payload sizes</p>

                    <h4 class="text-red-400">Con</h4>
                    <p class="text-red-400">Larger url safe payload sizes</p>
                    <p class="text-red-400">Larger and less compatible</p>
                </figure>

                <p>
                    On small to medium websites the disadvantages above are
                    negligible. For large websites though, which are conscious
                    of both HTTP payload sizes and server side storage usage,
                    with identifiers been passed around all the time, having the
                    best of both worlds would be ideal.
                </p>

                <h2 class="text-2xl border-b-2 border-gray-700 mt-4">
                    A Solution
                </h2>

                <p>
                    This website serves to demonstrate and explain <a
                        href="https://crates.io/crates/base64id">Base64id-rs</a
                    >, a Rust library for converting 64, 32 and 16 bit integers
                    to and from base64url encoded strings. The goal of this
                    library is to provide the best of both worlds.
                </p>

                <p>
                    On the backend you generate your identifier as an integer.
                    This integer can be used in memory as needed and then stored
                    on a database. Then when it needs to be sent to a client via
                    HTTP, the identifier can be converted to it’s base64url
                    equivalent and sent as a string. When the identifier must be
                    sent back to the server, the server can receive the
                    base64url string and decode it back into an integer.
                </p>

                <p>
                    This exchange uses fewer bytes than if transferring the
                    integer as is, while still benefiting from the use of an
                    integer server side, both in memory and in your database of
                    choice.
                </p>
            </div>
        </div>
    </article>
</main>

<style lang="postcss">
    mono {
        @apply font-mono;
    }
</style>

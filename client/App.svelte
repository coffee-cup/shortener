<script lang="ts">
  import Tailwindcss from "./Tailwind.svelte"

  let url = "";
  let shortenedUrl = "asdfasdf";
  let error = "";

  async function handleSubmit() {
    if (url.trim() === "") return;

    shortenedUrl = "";
    error = "";

    const res = await fetch("/", { 
      method: "POST", 
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ url: url.startsWith("http") ? url : `https://${url}` })
    });
    const text = await res.text()

    if (res.ok) {
      shortenedUrl = text;
    } else {
      error = text;
    }
  }

  function handleAnother() {
    shortenedUrl = "";
    error = "";
  }
</script>

<main class="px-4 min-h-screen flex flex-col items-center justify-center">
  <header class="text-center space-y-4 mb-8">
    <h1 class="text-5xl font-bold">Link Shortener</h1>

    <h2 class="text-xl font-medium">
    {#if shortenedUrl === ""}
      Enter a URL to shorten
    {:else}
      Here you go 👇
    {/if}
    </h2>

  </header>

  {#if shortenedUrl !== ""}
    <a class="block break-all text-xl md:text-3xl font-bold font-mono text-primary underline" href={shortenedUrl} target="_blank">
      {shortenedUrl.replace(/https?:\/\//, "")}
    </a>

    <button on:click={handleAnother} class="rounded bg-secondary hover:shadow text-foreground px-2 mt-4">Another</button>
  {:else if error !== ""}
    <div class="block text-xl md:text-3xl font-bold font-mono text-primary">
      {error}
    </div>
  {:else}
    <form on:submit|preventDefault={handleSubmit} class="flex space-x-2 w-full max-w-lg mx-auto">
      <input bind:value={url} class="rounded border-primary px-4 py-2 text-xl w-full flex-grow focus:outline-none focus:ring-2 focus:ring-secondary" placeholder="example.com" />

      <button class="rounded bg-primary hover:shadow text-foreground py-2 px-4 text-base font-bold focus:outline-none focus:ring-2 focus:ring-secondary">Shorten!</button>
    </form>
  {/if}

</main>

<footer class="bg-tertiary">
<div class="max-w-2xl mx-auto px-4 py-8 flex flex-col items-center space-y-2 md:flex-row md:justify-between md:space-x-4 md:space-y-0">
  <span>Created with ♥ by <a href="https://jakerunzer.com" class="hover:opacity-60 underline">Jake Runzer</a></span>
  <span>Source on <a href="https://github.com/coffee-cup/shortener" class="hover:opacity-60 underline">GitHub</a></span>
</div>
</footer>

<Tailwindcss />

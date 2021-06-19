<script lang="ts">
  import Tailwindcss from "./Tailwind.svelte"

  let url = "";
  let shortenedUrl = "";
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
</script>

<main class="bg-background text-foreground px-4 min-h-screen flex flex-col items-center justify-center">
  <header class="text-center space-y-4 mb-8">
    <h1 class="text-5xl font-bold">Link Shortener</h1>

    {#if shortenedUrl === ""}
      <h2 class="text-xl font-medium">Enter a URL to shorten</h2>
    {:else}
      <h2 class="text-xl font-medium">Here you go 👇</h2>
    {/if}

  </header>

  {#if shortenedUrl !== ""}
    <a class="block break-all text-xl md:text-3xl font-bold font-mono text-primary underline" href={shortenedUrl} target="_blank">
      {shortenedUrl.replace(/https?:\/\//, "")}
    </a>
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


<Tailwindcss />

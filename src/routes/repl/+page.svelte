<script>
    import { onMount } from "svelte";
    import Terminal from "$lib/components/terminal/terminal.svelte";

    onMount(async () => {
      console.log('On repl page!');
    });

</script>

<Terminal />

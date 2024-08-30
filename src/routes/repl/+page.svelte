<script>
    import { onMount } from "svelte";
    import Terminal from "../../components/terminal.svelte";

    onMount(async () => {
      console.log('On repl page!');
    });

</script>

<Terminal />

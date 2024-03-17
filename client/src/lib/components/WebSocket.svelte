<script lang="ts">
    import { onMount, onDestroy } from 'svelte';

    let ws: WebSocket;

    onMount(() => {
        ws = new WebSocket('ws://localhost:8080');

        ws.onopen = () => {
            console.log('Connected to the WebSocket server');
            return false;
        };

        ws.onmessage = (event: MessageEvent) => {
            console.log('Message from server', event.data);
            // Use event.data after checking its type if needed
        };

        ws.onerror = (event: Event) => {
            console.error('WebSocket error', event);
        };

        ws.onclose = (event) => {
            console.log(`WebSocket connection closed with code: ${event.code}, reason: ${event.reason}`);
        };
    });

    onDestroy(() => {
        if (ws) ws.close(); // Ensure ws is not undefined before calling close
    });
</script>
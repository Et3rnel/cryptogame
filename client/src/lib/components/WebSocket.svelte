<script lang="ts">
    import { onMount, onDestroy } from 'svelte';

    let ws: WebSocket;

    onMount(() => {
        ws = new WebSocket('ws://localhost:8080');
        let heartbeatInterval: number;
        const heartbeatFrequency = 30000; // Frequency of heartbeat in milliseconds (e.g., 30 seconds)

        ws.onopen = () => {
            console.log('Connected to the WebSocket server');
            return false;


            // heartbeatInterval = setInterval(() => {
            //     // Check if the WebSocket connection is still open
            //     if (ws.readyState === WebSocket.OPEN) {
            //         ws.send(JSON.stringify({ type: 'heartbeat', data: 'ping' }));
            //     }
            // }, heartbeatFrequency);
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
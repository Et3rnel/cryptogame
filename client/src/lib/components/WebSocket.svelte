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

        document.addEventListener('keydown', event => {
            switch (event.key) {
                case 'ArrowUp':
                    sendMove(0x01);
                    break;
                case 'ArrowDown':
                    sendMove(0x02);
                    break;
                case 'ArrowLeft':
                    sendMove(0x03);
                    break;
                case 'ArrowRight':
                    sendMove(0x04);
                    break;
            }
        });
    });

    onDestroy(() => {
        if (ws) ws.close(); // Ensure ws is not undefined before calling close
    });

    function sendMove(directionCode: number) {
        const buffer = new ArrayBuffer(2); // 2 bytes: 1 for action, 1 for direction
        const view = new DataView(buffer);
        view.setUint8(0, 0x01); // 0x01 for "move" action
        view.setUint8(1, directionCode); // direction code
        ws.send(buffer);
    }
</script>
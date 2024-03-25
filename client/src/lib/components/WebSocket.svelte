<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { MOVE_COMMAND } from "$lib/commands";

    let ws: WebSocket;
    let canvas: HTMLCanvasElement;
    let context: CanvasRenderingContext2D | null;
    let currentDirection: string | null;

    onMount(() => {
        ws = new WebSocket('ws://localhost:8080');

        ws.onopen = () => {
            console.log('Connected to the WebSocket server');
            return false;
        };

        ws.onmessage = (event: MessageEvent) => {
            handleWebSocketMessage(event);
        };

        ws.onerror = (event: Event) => {
            console.error('WebSocket error', event);
        };

        ws.onclose = (event) => {
            console.log(`WebSocket connection closed with code: ${event.code}, reason: ${event.reason}`);
        };

        document.addEventListener('keydown', event => {
            switch (event.key) {
                case 'ArrowLeft':
                    currentDirection = 'left';
                    break;
                case 'ArrowRight':
                    currentDirection = 'right';
                    break;
            }
        });

        document.addEventListener('keyup', event => {
            if ((event.key === 'ArrowLeft' && currentDirection === 'left') ||
                (event.key === 'ArrowRight' && currentDirection === 'right')) {
                currentDirection = null;
            }
        });

        setInterval(() => {
            if (currentDirection === 'left') {
                sendMove(0x01);
            } else if (currentDirection === 'right') {
                sendMove(0x02);
            }
        }, 16); // TODO: should we send the command each 16 ms since it's our server tick rate ?

        context = canvas.getContext('2d');
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

    function drawPlayer(x: number, y: number): void {
        if (context) {
            context.beginPath();
            context.arc(x, y, 10, 0, 2 * Math.PI);
            context.fillStyle = 'red';
            context.fill();
        }
    }

    function handleWebSocketMessage(event: MessageEvent) {
        if (!(event.data instanceof Blob)) return;

        const reader = new FileReader();
        reader.onload = function() {
            // `reader.result` can be `string | ArrayBuffer | null`, we make sure it's an array buffer
            if (typeof reader.result === 'string' || reader.result === null) {
                console.error('Expected an ArrayBuffer');
                return;
            }

            const buffer = new Uint8Array(reader.result);
            const commandId = buffer[0];

            if (commandId === MOVE_COMMAND) {
                const dataView = new DataView(reader.result);
                const posX = dataView.getFloat64(1, false); // big-endian
                const posY = dataView.getFloat64(9, false); // big-endian

                // console.log('X: ' + posX + ', Y: ' + posY)

                drawPlayer(posX, posY);
            }
        };
        reader.readAsArrayBuffer(event.data);
    }
</script>

<style>
    canvas {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        border: 1px solid #08ff00;
    }
</style>


<canvas bind:this={canvas} width="700" height="700"></canvas>
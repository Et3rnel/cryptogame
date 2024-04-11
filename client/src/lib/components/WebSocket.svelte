<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { MOVE_COMMAND } from "$lib/commands";

    let ws: WebSocket;
    let canvas: HTMLCanvasElement;
    let context: CanvasRenderingContext2D | null;
    let currentDirection: string | null;

    onMount(() => {
        ws = new WebSocket('ws://127.0.0.1:8080');

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

    function drawPlayer(uuid: string, x: number, y: number, color: string): void {
        if (context) {
            context.beginPath();
            context.arc(x, y, 10, 0, 2 * Math.PI);
            context.fillStyle = color;
            context.fill();
        }
    }

    const playerColors: any = {};

    function getRandomColor(): any {
        // Generates a random hex color, avoiding white
        let color = "#";
        for (let i = 0; i < 3; i++) {
            // Generate a value between 0 and 255, converted to a hex string
            const part = Math.floor(Math.random() * 256).toString(16).padStart(2, '0');
            color += part;
        }
        return color !== "#FFFFFF" ? color : getRandomColor(); // Ensure color is not white
    }

    function handleWebSocketMessage(event: MessageEvent) {
        if (!(event.data instanceof Blob)) return;

        const reader = new FileReader();
        reader.onload = function() {
            if (typeof reader.result === 'string' || reader.result === null) {
                console.error('Expected an ArrayBuffer');
                return;
            }

            const buffer = new Uint8Array(reader.result);
            // Assuming the first byte is a command ID, you can ignore or use it as needed
            const commandId = buffer[0];
            // console.log('Totoo');
            // Ensure this is the command we are interested in for position updates
            if (commandId === MOVE_COMMAND) {

                // Iterate through each player in the message
                for (let offset = 1; offset < buffer.length; offset += 32) {
                    // Extract the UUID (as a hexadecimal string for simplicity)
                    const uuidBytes = buffer.slice(offset, offset + 16);
                    const playerId = [...uuidBytes].map(b => b.toString(16).padStart(2, '0')).join('');

                    // Assign a random color to the player if it doesn't have one
                    if (!playerColors[playerId]) {
                        playerColors[playerId] = getRandomColor();
                    }

                    // Extract the x and y positions
                    const dataView = new DataView(buffer.buffer, offset + 16, 16); // Use buffer.buffer to get the underlying ArrayBuffer
                    const posX = dataView.getFloat64(0, false); // big-endian
                    const posY = dataView.getFloat64(8, false); // big-endian

                    // Draw the player using their specific color
                    drawPlayer(playerId, posX, posY, playerColors[playerId]);
                }
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


<canvas bind:this={canvas} width="600" height="600"></canvas>
<script lang="ts">
	import {onMount} from "svelte";

	let isCameraActive = false;
	let capturedImage: string | null = null;
	let stream: MediaStream | null = null;
	let videoElement: HTMLVideoElement;
	let canvasElement: HTMLCanvasElement;

	async function startCamera() {
		try {
			stream = await navigator.mediaDevices.getUserMedia({
                audio: false,
                video: { facingMode: 'environment' }
			});
			videoElement.srcObject = stream;
			isCameraActive = true;
		} catch (error) {
			console.error('Error starting camera:', error);
		}
	}

	function takePicture() {
		if (videoElement && canvasElement) {
			const context = canvasElement.getContext('2d');
			canvasElement.width = videoElement.videoWidth;
			canvasElement.height = videoElement.videoHeight;
			context?.drawImage(videoElement, 0, 0);
			capturedImage = canvasElement.toDataURL('image/png');
			stopCamera();
		}
	}

	function stopCamera() {
		if (stream) {
			stream.getTracks().forEach(track => track.stop());
			isCameraActive = false;
		}
	}

	function downloadPicture() {
		if (capturedImage) {
			const a = document.createElement('a');
			a.href = capturedImage;
			a.download = `litter-${Date.now()}.png`;
			a.click();
		}
	}

	onMount(() => startCamera())
</script>

<div class="h-center flex flex-col justify-center items-center">

    <div class="flex-row mb-8">
        <video bind:this={videoElement} playsinline
               muted
               autoplay style="display: {isCameraActive ? 'block' : 'none'}"></video>
        <canvas bind:this={canvasElement} style="display: none;"></canvas>

        {#if capturedImage}
            <img src={capturedImage} alt="Captured litter" />
        {/if}
    </div>

    <div class="controls flex-row">
        {#if !isCameraActive && !capturedImage}
            <button class="btn btn-secondary flex justify-center" on:click={startCamera}>Allow Camera</button>
        {:else if isCameraActive}
            <button class="btn btn-secondary" on:click={takePicture}>
                <svg class="size-[1.2em]" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                     stroke="currentColor">
                    <path stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                          d="M6.827 6.175A2.31 2.31 0 0 1 5.186 7.23c-.38.054-.757.112-1.134.175C2.999 7.58 2.25 8.507 2.25 9.574V18a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18V9.574c0-1.067-.75-1.994-1.802-2.169a47.865 47.865 0 0 0-1.134-.175 2.31 2.31 0 0 1-1.64-1.055l-.822-1.316a2.192 2.192 0 0 0-1.736-1.039 48.774 48.774 0 0 0-5.232 0 2.192 2.192 0 0 0-1.736 1.039l-.821 1.316Z"/>
                    <path stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                          d="M16.5 12.75a4.5 4.5 0 1 1-9 0 4.5 4.5 0 0 1 9 0ZM18.75 10.5h.008v.008h-.008V10.5Z"/>
                </svg>
                Take Picture
            </button>
        {/if}

        {#if capturedImage}
            <button on:click={downloadPicture}>Download Picture</button>
            <button on:click={() => { capturedImage = null; startCamera(); }}>Take Another</button>
        {/if}
    </div>

</div>
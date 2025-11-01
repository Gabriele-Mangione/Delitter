<script lang="ts">
	let isCameraActive = false;
	let capturedImage: string | null = null;
	let stream: MediaStream | null = null;
	let videoElement: HTMLVideoElement;
	let canvasElement: HTMLCanvasElement;

	async function startCamera() {
		try {
			stream = await navigator.mediaDevices.getUserMedia({audio: false, video: true});
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
</script>

<div class="recording-page">
    <h1>Take litter picture</h1>

    <video bind:this={videoElement} playsinline
           muted
           autoplay style="display: {isCameraActive ? 'block' : 'none'}"></video>
    <canvas bind:this={canvasElement} style="display: none;"></canvas>

	{#if capturedImage}
		<img src={capturedImage} alt="Captured litter" />
	{/if}

    <div class="controls">
        {#if !isCameraActive && !capturedImage}
            <button on:click={startCamera}>Start Camera</button>
        {:else if isCameraActive}
            <button on:click={takePicture}>Take Picture</button>
            <button on:click={stopCamera}>Cancel</button>
        {/if}

        {#if capturedImage}
            <button on:click={downloadPicture}>Download Picture</button>
			<button on:click={() => { capturedImage = null; startCamera(); }}>Take Another</button>
        {/if}
    </div>
</div>
import * as mc_m2p from 'mc_map2png';

document.getElementById('convertButton').addEventListener('click', async () => {
	const fileInput = document.getElementById('fileInput');
	const outputImage = document.getElementById('outputImage');
	const downloadButton = document.getElementById('downloadButton');
	const conversionTime = document.getElementById('conversionTime');
	
	if (fileInput.files.length === 0) {
		alert('Please select a file.');
		return;
	}

	const file = fileInput.files[0];
	const arrayBuffer = await file.arrayBuffer();
	const startTime = performance.now();
	const result = await mc_m2p.process_image_from_memory(new Uint8Array(arrayBuffer));
	const endTime = performance.now();
	
	const timeTaken = (endTime - startTime).toFixed(2);
	conversionTime.textContent = `Conversion time: ${timeTaken} ms`;

	if (result instanceof Uint8Array) {
		const blob = new Blob([result], {type: 'image/png'});
		const url = URL.createObjectURL(blob);
		outputImage.src = url;
		outputImage.classList.remove('hidden');
		downloadButton.classList.remove('hidden');
		downloadButton.onclick = function() {
			const link = document.createElement('a');
			link.href = url;
			link.download = 'converted-image.png';
			document.body.appendChild(link);
			link.click();
			document.body.removeChild(link);
		};
	} else {
		alert('Error processing file');
	}
});
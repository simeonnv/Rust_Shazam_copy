<script setup lang="ts">
import MainIcon from "@/assets/MainIcon.vue";
import { MediaRecorder, register } from "extendable-media-recorder";
import { connect } from "extendable-media-recorder-wav-encoder";
import { ref } from "vue";

const stream = ref();
const matches = ref([]);
const totalSongs = ref(10);
const isListening = ref(false);
const audioInput = ref("device"); // or "mic"
const isPhone = ref(window.innerWidth <= 550);
const registeredMediaEncoder = ref(false);
const sendRecordingRef = ref(true);

async function record() {
    try {
        const mediaDevice =
            audioInput.value === "device"
                ? navigator.mediaDevices.getDisplayMedia.bind(
                      navigator.mediaDevices,
                  )
                : navigator.mediaDevices.getUserMedia.bind(
                      navigator.mediaDevices,
                  );

        if (!registeredMediaEncoder) {
            await register(await connect());
            registeredMediaEncoder.value = true;
        }

        const constraints = {
            audio: {
                autoGainControl: false,
                channelCount: 1,
                echoCancellation: false,
                noiseSuppression: false,
                sampleSize: 16,
            },
        };

        const stream = await mediaDevice(constraints);
        const audioTracks = stream.getAudioTracks();
        const audioStream = new MediaStream(audioTracks);

        stream.value = audioStream;

        audioTracks[0].onended = stopListening;

        // Stop video tracks
        for (const track of stream.getVideoTracks()) {
            track.stop();
        }

        const mediaRecorder = new MediaRecorder(audioStream, {
            mimeType: "audio/wav",
        });

        mediaRecorder.start();
        isListening.value = true;
        sendRecordingRef.value = true;

        const chunks = [];
        mediaRecorder.ondataavailable = (e) => {
            chunks.push(e.data);
        };

        // Stop recording after 20 seconds
        setTimeout(() => {
            mediaRecorder.stop();
        }, 20000);

        mediaRecorder.addEventListener("stop", () => {
            const blob = new Blob(chunks, { type: "audio/wav" });
            const reader = new FileReader();

            cleanUp();

            reader.readAsArrayBuffer(blob);
            reader.onload = async (event: any) => {
                const arrayBuffer = event.target.result;

                // get record duration
                const arrayBufferCopy = arrayBuffer.slice(0);
                const audioContext = new AudioContext();
                const audioBufferDecoded =
                    await audioContext.decodeAudioData(arrayBufferCopy);
                const recordDuration = audioBufferDecoded.duration;

                var binary = "";
                var bytes = new Uint8Array(arrayBuffer);
                var len = bytes.byteLength;
                for (var i = 0; i < len; i++) {
                    binary += String.fromCharCode(bytes[i]);
                }

                // Convert byte array to base64
                const rawAudio = btoa(binary);
                const audioConfig = audioStream
                    .getAudioTracks()[0]
                    .getSettings();

                const recordData = {
                    audio: rawAudio,
                    duration: recordDuration,
                    channels: audioConfig.channelCount,
                    sampleRate: audioConfig.sampleRate,
                    sampleSize: audioConfig.sampleSize,
                };
            };
        });
    } catch (error) {
        console.error("error:", error);
        cleanUp();
    }
}

function downloadRecording(blob) {
    const blobUrl = URL.createObjectURL(blob);

    // Create a download link
    const downloadLink = document.createElement("a");
    downloadLink.href = blobUrl;
    downloadLink.download = "recorded_audio.wav";
    document.body.appendChild(downloadLink);
    downloadLink.click();
}

function cleanUp() {
    const currentStream = stream.value;
    if (currentStream) {
        currentStream.getTracks().forEach((track) => track.stop());
    }

    stream.value = null;
    isListening.value = false;
}

function stopListening() {
    cleanUp();
    sendRecordingRef.value = false;
}

function handleLaptopIconClick() {
    audioInput.value = "device";
}

function handleMicrophoneIconClick() {
    audioInput.value = "mic";
}
</script>

<template>
    <div
        class="flex flex-row w-full h-screen items-center justify-center align-middle bg-cyan-500"
    >
        <div
            class="flex flex-col gap-4 w-full justify-center items-center text-emerald-600 font-bold text-2xl"
        >
            <div class="relative border rounded-full">
                <span
                    class="absolute inline-flex h-full w-full animate-ping rounded-full bg-sky-400"
                ></span>
                <MainIcon class="w-32 h-32 text-emerald-600 rotate-180" />
            </div>
            <p class="">listen</p>
        </div>
    </div>
</template>

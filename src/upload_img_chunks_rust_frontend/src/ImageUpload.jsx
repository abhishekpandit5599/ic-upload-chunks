import React, { useRef, useState } from 'react';
import { upload_img_chunks_rust_backend } from "../../declarations/upload_img_chunks_rust_backend/index.js";

const ImageUploader = () => {
    const videoRef = useRef();
    const [imageFile, setImageFile] = useState(null);
    const [chunks, setChunks] = useState([]);

    const handleImageChange = (event) => {
        const file = event.target.files[0];
        if (file) {
            setImageFile(file);
        }
    };

    const uploadImage = async () => {
        if (imageFile) {
            const reader = new FileReader();
            reader.onload = async () => {
                const buffer = reader.result;
                await divideBufferIntoChunks(buffer);
            };
            reader.readAsArrayBuffer(imageFile);
        }
    };

    const divideBufferIntoChunks = async (buffer) => {
        const chunkSize = 1 * 1024 * 1024; // 1MB chunk size
        const chunks = [];
        let j = 0;
        for (let i = 0; i < buffer.byteLength; i += chunkSize) {
            j+=1;
            let chunk = buffer.slice(i, i + chunkSize);
            chunk = Array.from(new Uint8Array(chunk));
            let data = await upload_img_chunks_rust_backend.upload_image(5,j,chunk);
            chunks.push(chunk);
        }
        // Now you can do something with these chunks, like sending them to the server
        console.log("chunks",chunks);
        // setChunks(chunks);
    };

    const previewImage = () => {
        if (chunks.length > 0) {
            const blob = new Blob(chunks, { type: "image/jpg" });
            const reader = new FileReader();
            reader.onload = () => {
                const imageDataUrl = reader.result;
                // Display the image preview
                document.getElementById('imagePreview').src = imageDataUrl;
            };
            reader.readAsDataURL(blob);
        }
    };
    const previewVideo = () => {
        if (videoRef.current && chunks.length > 0) {
            const blob = new Blob(chunks, { type: 'video/mp4' });
            const url = URL.createObjectURL(blob);
            videoRef.current.src = url;
            videoRef.current.play();
          }
    };

    const getImage = async () => {
        try {
            let i = 1;
            let data;
            let chunks = [];
            do{
                console.log("i",i)
                data = await upload_img_chunks_rust_backend.get_image(5,i);
                console.log("data ====>",data)
                if(data.length > 0){
                    data = Buffer.from(data?.[0])
                    console.log("data",data)
                    i+=1;
                    chunks.push(data);
                }
            }while(data.byteLength);
            setChunks(chunks);
            console.log("chunks =================================>",chunks);
        } catch (error) {
            console.log("chunks =================================>",chunks);
        }
    }

    return (
        <div>
            <input type="file" onChange={handleImageChange} />
            <button onClick={uploadImage}>Upload Image</button>
            <button onClick={getImage}>Get Image</button>
            <button onClick={previewImage}>Preview Image</button>
            <button onClick={previewVideo}>Preview Video</button>
            <br />
            <img id="imagePreview" alt="Preview" />
            <video ref={videoRef} controls />
        </div>
    );
};

export default ImageUploader;

import { useState } from 'react';
import { upload_img_chunks_rust_backend } from 'declarations/upload_img_chunks_rust_backend';
import ImageUploader from './ImageUpload';

function App() {
  const [greeting, setGreeting] = useState('');

  function handleSubmit(event) {
    event.preventDefault();
    const name = event.target.elements.name.value;
    upload_img_chunks_rust_backend.greet(name).then((greeting) => {
      setGreeting(greeting);
    });
    return false;
  }

  return (
    <main>
      <ImageUploader />
    </main>
  );
}

export default App;

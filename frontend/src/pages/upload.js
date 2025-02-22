import React from 'react';
import { useEffect } from 'react';
import { useRouter } from 'next/router';
import { useMutation, QueryClient } from 'react-query';
import axios from 'axios';

const queryClient = new QueryClient();

function uploadPage() {
    const router = useRouter();
    const { mutate } = useMutation(
    (formData) => axios.post('http://127.0.0.1:5353/api/upload', formData),
    {
      headers: {"Content-Type": "multipart/form-data"},
      onSuccess: () => { // If the upload is successful:
        // Don't refetch 
        alert("File uploaded");
        queryClient.invalidateQueries('files');
        // Redirect to the home page
        router.push('/');
      },
      onError: (error) => {
        console.error('Error uploading file:', error);
        alert('Error uploading file.');
      },
    }
  );
  useEffect(() => {
    // This code runs only in the browser after the component mounts
    const fileInput = document.getElementById('fileInput');
    const uploadButton = document.getElementById('uploadButton');

    uploadButton.addEventListener('click', () => {
      if (fileInput.files.length === 0) {
        alert('Please select a file to upload.');
        return;
      }

      const file = fileInput.files[0];
      const formData = new FormData();
      formData.append('file', file);

      // Call the mutate function with the FormData
      mutate(formData);
    });
  }, []);

  return (
    <div>
      <h1>Upload</h1>
      <input type="file" id="fileInput" />
      <button id="uploadButton">Upload</button>
      <div className="preview"></div>
    </div>
  );
}


export default uploadPage;

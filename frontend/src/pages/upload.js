import react from 'react';
import reactDom from 'react-dom';
import next from 'next';
import axios from 'axios';
import { useRouter } from 'next/router';
import { useQueryClient, QueryClient, useMutation, useQuery, QueryClientProvider } from 'react-query';

export { react, reactDom };
const queryClient = new QueryClient();
function uploadPage() {
    const router = useRouter(); 
    const { mutate } = useMutation(
        (data) => axios.post('http://localhost:5353/api/upload', data),
        {
            onSuccess: () => {
                queryClient.invalidateQueries('files');
                router.push('/');
            }
        }
    );
    
    return (
        <form onSubmit={(e) => {
            e.preventDefault();
            const formData = new FormData(e.target);
            mutate(formData);
        }}
        >
        <input type="file" name="file" />
        <button type="submit">Upload</button>
        </form>
    );
}

export default uploadPage;

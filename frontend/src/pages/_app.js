import React from "react";
import { QueryClient, QueryClientProvider } from "react-query";
import MyNavbar from "./Navbar";
import "./styles.css";
import 'bootstrap/dist/css/bootstrap.min.css';

const queryClient = new QueryClient();

function MyApp({ Component, pageProps }) {
  return (
    <QueryClientProvider client={queryClient}>
      <MyNavbar />
      <Component {...pageProps} />
    </QueryClientProvider>
  );
}

export default MyApp;
import React from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import { Link } from 'react-router-dom'
import NavBar from "./Navbar";

export default function Home() {
  return (
    <>
  <div style={{
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    height: '100vh'
    }}>
    <h1>Welcome to DiseaseLLM Project!</h1>
    
  </div>
  </>)
}


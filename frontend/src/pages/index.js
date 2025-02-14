import React from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import { Link } from 'react-router-dom'

export default function Home() {
  return (<div style={{
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    height: '100vh'
    }}>
    <h1>Welcome to DiseaseLLM Project!</h1>
    <a style={{position: "fixed",
    top: 0,
    left: 0,
    fontSize: "24px"}}href="/signup">Signup</a>
  </div>)
}

// import React from "react";
// import ReactDOM from "react-dom/client";
// import {
//   Route,
//   RouterProvider,
//   BrowserRouter,
//   Switch,
//   createBrowserRouter,
//   createRoutesFromElements
// } from "react-router-dom";


// import Navbar from "./Navbar";
// import Signup from "./signup";
// import Home from "./home";

// export default function Indexes() {
//   return(
//     <BrowserRouter>
//       <Switch>
//       <Route exact path="/" render={() => <Redirect to="/home" />} />
//       <Route path="/home" component={Home} />
//       <Route path="/signup" component={Signup} />
//       </Switch>
//     </BrowserRouter>
//     );
// }


// const router = createBrowserRouter(
//   createRoutesFromElements(
//     <>
//     <Route path="/" element={<Navbar />}>
//       <Route index element={<Home />} />
//       <Route path="signup" element={<Signup />} />
//     </Route>
//     </>
    
//   )
// );


// ReactDOM.createRoot(document.getElementById("root")).render(
//   <React.StrictMode>
//     <RouterProvider router={router} />
//   </React.StrictMode>
// );

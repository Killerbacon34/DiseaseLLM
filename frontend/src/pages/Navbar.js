import { Link } from "react-router-dom";

const Navbar = () => {
  return (
    <nav>
      <div>
        <Link to="/Home">Home</Link>
        <div className="space-x-4">
          <Link to="/upload">Upload File</Link>
          <Link to="/signup">Signup</Link>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;


// import { Link, Outlet } from "react-router-dom";

// export default function Navbar() {
//   return (
//     <>
//       <header className="Navbar">
//         <nav>
//           <ul>
//             <li>
//               <Link href="/">Home</Link>
//             </li>
//             <li>
//               <Link href="signup">Signup</Link>
//             </li>
//           </ul>
//         </nav>
//       </header>
//       <main>
//         <Outlet />
//       </main>
//     </>
//   );
// }
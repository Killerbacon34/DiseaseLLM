import Link from "next/link";


const Navbar = () => {
  return (
    <nav style={{backgroundcolor: 'blue',
    display: 'flex',
    justifyContent: 'space-between',
    padding: '1rem',
    alignItems: 'center',
    color: 'white'
    }}>
      <div style={{display: 'flex', alignItems: 'center', fontWeight: 'bold', fontSize: '1.5rem'}}>
        <Link href="/">Home</Link>
      </div>
      <div style={{display: 'flex', marginRight: '1rem', fontWeight: 'bold', fontSize: '1.5rem'}}>
        <Link href="/signup">Sign Up</Link>
      </div>
      <div style={{display: 'flex', marginRight: '1rem', fontWeight: 'bold', fontSize: '1.5rem'}}>
        <Link href="/upload">Upload</Link>
      </div>
    </nav>
  );
};

export default Navbar;

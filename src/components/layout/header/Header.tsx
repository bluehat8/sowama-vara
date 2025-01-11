import { Wallet } from '@gear-js/wallet-connect';
import { useEffect, useState } from "react";
import styles from './Header.module.scss';
import { Logo } from './logo';

function Header() {

  const [isScrolled, setIsScrolled] = useState(false);

  useEffect(() => {
    const handleScroll = () => {
      if (window.scrollY > 0) {
        setIsScrolled(true);
      } else {
        setIsScrolled(false);
      }
    };

    window.addEventListener("scroll", handleScroll);

    // Limpieza del evento cuando el componente se desmonta
    return () => {
      window.removeEventListener("scroll", handleScroll);
    };
  }, []);

  return (
    // <header className={styles.header}>
    //   <Logo />
    //   <Wallet />
    // </header>

    <header
    className={`fixed top-0 left-0 w-full z-50 transition-all ${
      isScrolled ? "abajo" : ""
    }`}
  >
    <div className="container mx-auto flex items-center justify-between py-4 px-6">
      <a href="/" className="flex items-center">
        <img src="/logo.svg" alt="Logo" className="w-10 h-10" />
        <span className="ml-3 text-lg font-semibold text-white">SOWAMA</span>
      </a>

      <nav className="flex gap-4">
        {/* Enlace Home */}
        <a
          href="/"
          className="text-white hover:text-secondary font-semibold"
        >
          Home
        </a>

        {/* Enlace Dashboard */}
        {/* {isConnected && (
          <a
            href="/dashboard"
            className="text-white hover:text-secondary font-semibold"
          >
            Dashboard
          </a>
        )} */}

        {/* Botón Conectar Wallet */}
        <Wallet />
      </nav>
    </div>
  </header>
  );
}

export { Header };

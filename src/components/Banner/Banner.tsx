// src/components/Banner.tsx
import React from 'react';
import './Banner.scss'; // Asegúrate de agregar un archivo SCSS para el banner.

function Banner() {
  return (
    <section className="banner">
      <div className="banner-content">
        <h1 className="banner-title">Welcome to SOWAMA</h1>
        <p className="banner-description">AN EVERGREEN WORLD</p>
      </div>
    </section>
  );
}

export { Banner };

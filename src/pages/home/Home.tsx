// src/pages/home.tsx
import React from 'react';
import { Banner } from '../../components/Banner/Banner';

function Home() {
  return (
    <div>
      <Banner />
      {/* Aquí puede ir el contenido principal de la página */}
      <div className="content text-center">
        {/* <h2>Contenido de la Página Principal</h2> */}
        {/* Aquí puedes añadir más componentes o contenido */}
      </div>
    </div>
  );
}

export { Home };


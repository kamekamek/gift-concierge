'use client';

import Chat from './components/Chat';
import GiftRecommendations from './components/GiftRecommendations';
import { useState } from 'react';

export default function Home() {
  const [gifts, setGifts] = useState([]);

  return (
    <main className="min-h-screen bg-gray-100">
      <div className="container mx-auto px-4 py-8">
        <h1 className="text-3xl font-bold text-center mb-8">
          お返しコンシェルジュ
        </h1>
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          <div className="bg-white rounded-lg shadow-lg overflow-hidden">
            <Chat />
          </div>
          <div className="bg-white rounded-lg shadow-lg overflow-hidden">
            <GiftRecommendations gifts={gifts} />
          </div>
        </div>
      </div>
    </main>
  );
}

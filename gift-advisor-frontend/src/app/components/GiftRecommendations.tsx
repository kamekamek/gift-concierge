import { useState } from 'react';
import Image from 'next/image';

interface Gift {
  name: string;
  price: number;
  description: string;
  image_url: string | null;
  category: string;
  rating: number;
  source: string;
}

interface GiftRecommendationsProps {
  gifts: Gift[];
}

export default function GiftRecommendations({ gifts }: GiftRecommendationsProps) {
  const [selectedGift, setSelectedGift] = useState<Gift | null>(null);

  return (
    <div className="p-4">
      <h2 className="text-2xl font-bold mb-4">おすすめのギフト</h2>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {gifts.map((gift, index) => (
          <div
            key={index}
            className="bg-white rounded-lg shadow-lg overflow-hidden hover:shadow-xl transition-shadow"
          >
            <div className="relative h-48">
              {gift.image_url ? (
                <Image
                  src={gift.image_url}
                  alt={gift.name}
                  layout="fill"
                  objectFit="cover"
                />
              ) : (
                <div className="w-full h-full bg-gray-200 flex items-center justify-center">
                  <span className="text-gray-400">No image</span>
                </div>
              )}
            </div>
            <div className="p-4">
              <h3 className="text-lg font-semibold mb-2">{gift.name}</h3>
              <p className="text-gray-600 mb-2">{gift.description}</p>
              <div className="flex justify-between items-center mb-2">
                <span className="text-lg font-bold">¥{gift.price.toLocaleString()}</span>
                <span className="text-sm text-gray-500">{gift.category}</span>
              </div>
              <div className="flex items-center mb-2">
                <div className="flex">
                  {[...Array(5)].map((_, i) => (
                    <svg
                      key={i}
                      className={`w-4 h-4 ${
                        i < Math.floor(gift.rating)
                          ? 'text-yellow-400'
                          : 'text-gray-300'
                      }`}
                      fill="currentColor"
                      viewBox="0 0 20 20"
                    >
                      <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
                    </svg>
                  ))}
                </div>
                <span className="ml-2 text-sm text-gray-500">
                  {gift.rating.toFixed(1)}
                </span>
              </div>
              <div className="text-sm text-gray-500 mb-4">
                提供: {gift.source}
              </div>
              <button
                onClick={() => setSelectedGift(gift)}
                className="w-full bg-blue-500 text-white py-2 rounded-lg hover:bg-blue-600 transition-colors"
              >
                詳細を見る
              </button>
            </div>
          </div>
        ))}
      </div>

      {/* 詳細モーダル */}
      {selectedGift && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4">
          <div className="bg-white rounded-lg max-w-2xl w-full max-h-[90vh] overflow-y-auto">
            <div className="p-6">
              <div className="flex justify-between items-start mb-4">
                <h2 className="text-2xl font-bold">{selectedGift.name}</h2>
                <button
                  onClick={() => setSelectedGift(null)}
                  className="text-gray-500 hover:text-gray-700"
                >
                  <svg
                    className="w-6 h-6"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M6 18L18 6M6 6l12 12"
                    />
                  </svg>
                </button>
              </div>
              <div className="relative h-64 mb-4">
                {selectedGift.image_url ? (
                  <Image
                    src={selectedGift.image_url}
                    alt={selectedGift.name}
                    layout="fill"
                    objectFit="cover"
                    className="rounded-lg"
                  />
                ) : (
                  <div className="w-full h-full bg-gray-200 rounded-lg flex items-center justify-center">
                    <span className="text-gray-400">No image</span>
                  </div>
                )}
              </div>
              <p className="text-gray-600 mb-4">{selectedGift.description}</p>
              <div className="grid grid-cols-2 gap-4 mb-4">
                <div>
                  <span className="text-gray-500">価格:</span>
                  <span className="ml-2 font-bold">
                    ¥{selectedGift.price.toLocaleString()}
                  </span>
                </div>
                <div>
                  <span className="text-gray-500">カテゴリー:</span>
                  <span className="ml-2">{selectedGift.category}</span>
                </div>
                <div>
                  <span className="text-gray-500">評価:</span>
                  <span className="ml-2">{selectedGift.rating.toFixed(1)}</span>
                </div>
                <div>
                  <span className="text-gray-500">提供:</span>
                  <span className="ml-2">{selectedGift.source}</span>
                </div>
              </div>
              <button
                onClick={() => window.open('#', '_blank')}
                className="w-full bg-blue-500 text-white py-2 rounded-lg hover:bg-blue-600 transition-colors"
              >
                商品を見る
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
} 
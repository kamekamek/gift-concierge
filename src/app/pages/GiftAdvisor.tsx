import React, { useState } from 'react';
import { Container, CircularProgress, Box } from '@mui/material';
import { GiftForm } from '../components/GiftForm';
import { GiftRecommendations } from '../components/GiftRecommendations';

interface GiftRecommendation {
  name: string;
  price: string;
  store: string;
  reason: string;
  etiquette_advice: string;
}

export const GiftAdvisor: React.FC = () => {
  const [loading, setLoading] = useState(false);
  const [recommendations, setRecommendations] = useState<GiftRecommendation[]>([]);

  const handleSubmit = async (formData: any) => {
    setLoading(true);
    try {
      const response = await fetch('/api/recommendations', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(formData),
      });

      if (!response.ok) {
        throw new Error('推奨ギフトの取得に失敗しました');
      }

      const data = await response.json();
      setRecommendations(data);
    } catch (error) {
      console.error('エラー:', error);
      // エラー処理を実装
    } finally {
      setLoading(false);
    }
  };

  return (
    <Container maxWidth="lg">
      <GiftForm onSubmit={handleSubmit} />
      
      {loading && (
        <Box display="flex" justifyContent="center" my={4}>
          <CircularProgress />
        </Box>
      )}
      
      {!loading && recommendations.length > 0 && (
        <GiftRecommendations recommendations={recommendations} />
      )}
    </Container>
  );
}; 
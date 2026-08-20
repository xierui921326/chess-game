import { describe, it, expect } from 'vitest';
import { fireEvent, render, screen } from '@testing-library/react';
import { MemoryRouter, Route, Routes } from 'react-router-dom';
import SetupPage from '../../pages/SetupPage';

function renderSetup(gameType: string) {
  return render(
    <MemoryRouter initialEntries={[`/setup/${gameType}`]}>
      <Routes>
        <Route path="/setup/:gameType" element={<SetupPage />} />
        <Route path="/play/:gameType" element={<div>play-page</div>} />
        <Route path="/" element={<div>home-page</div>} />
      </Routes>
    </MemoryRouter>,
  );
}

describe('SetupPage', () => {
  it('象棋设置页应展示执子与难度', () => {
    renderSetup('xiangqi');

    expect(screen.getByText('中国象棋')).toBeInTheDocument();
    expect(screen.getByText('执子颜色')).toBeInTheDocument();
    expect(screen.getByText('AI 难度')).toBeInTheDocument();
    expect(screen.getByText('简单')).toBeInTheDocument();
    expect(screen.getByText('普通')).toBeInTheDocument();
    expect(screen.getByText('困难')).toBeInTheDocument();
  });

  it('军棋设置页应标注翻棋', () => {
    renderSetup('junqi');

    expect(screen.getByText('军棋（翻棋）')).toBeInTheDocument();
    expect(screen.getByText(/二人暗棋翻棋/)).toBeInTheDocument();
  });

  it('非法棋种应回到主页', () => {
    renderSetup('go');
    expect(screen.getByText('home-page')).toBeInTheDocument();
  });

  it('开始对局应跳转到 play 路由', () => {
    renderSetup('xiangqi');

    fireEvent.click(screen.getByText('黑方'));
    fireEvent.click(screen.getByText('困难'));
    fireEvent.click(screen.getByText('开始对局'));

    expect(screen.getByText('play-page')).toBeInTheDocument();
  });
});

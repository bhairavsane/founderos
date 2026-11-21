import React, { useState } from 'react';
import { Sidebar } from './components/Sidebar';
import { UniversalSearch } from './components/UniversalSearch';
import { AskFounderOS } from './components/AskFounderOS';
import { Library } from './components/Library';
import { ModelsIndexing } from './components/ModelsIndexing';
import { NavTab } from './types';

export const App: React.FC = () => {
  const [currentTab, setCurrentTab] = useState<NavTab>('search');

  return (
    <div style={{ display: 'flex', width: '100vw', height: '100vh', overflow: 'hidden' }}>
      <Sidebar currentTab={currentTab} onSelectTab={setCurrentTab} />
      <main style={{ flex: 1, overflowY: 'auto', background: 'var(--bg)' }}>
        {currentTab === 'search' && <UniversalSearch />}
        {currentTab === 'ask' && <AskFounderOS />}
        {currentTab === 'library' && <Library />}
        {currentTab === 'models' && <ModelsIndexing />}
      </main>
    </div>
  );
};

export default App;

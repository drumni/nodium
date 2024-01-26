
import './Items.css'; // Import the CSS file
import ItemsContainer from './ItemsContainer';

interface ItemsProps {
  root: string;
  paths: string[];
  filters: string[];
  onToggleFolder: (path: string, isOpen: boolean) => void;
}

const Items = ({ root, paths, filters, onToggleFolder }: ItemsProps) => {
  return <ItemsContainer root={root} paths={paths} filters={filters} onToggleFolder={onToggleFolder} />;
};

export default Items;


// ItemRow.tsx
import React, { useEffect, useState } from "react";
import AnalyzeButton from "./AnalyzeButton";
import { invoke } from "@tauri-apps/api/tauri";

interface ItemRowProps {
 isFolder: boolean;
 isOpen: boolean;

 path: string;
 name: string;
 type: string;
 root: string;

 onClick: () => void;
}

const ItemRow: React.FC<ItemRowProps> = ({ isFolder, isOpen, path, name, type, onClick ,root}) => {
 const [count, setCount] = useState<number>(0);
 const [isLoading, setIsLoading] = useState<boolean>(false);
 const [isAnalyzed, setIsAnalyzed] = useState<boolean>(false);

 useEffect(() => {
    const checkAndReloadMarkdown = async () => {
      const hasMarkdown = await checkMarkdown();
      if (hasMarkdown) {
        await reloadMarkdown();
        setIsLoading(false);
      }
    };
    checkAndReloadMarkdown();
 }, []);

 const handleAnalyzeClick = async (e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
    e.stopPropagation();

    if (!isFolder) {
      // Check for markdown file
      const hasMarkdown = await checkMarkdown();

      if (hasMarkdown) {
        // Reload the markdown file (add your logic here)
        await reloadMarkdown();
        setIsLoading(false);
      }
    }
 };

 const checkMarkdown = async () => {
    // if no markdown path, generate it by removing root
    let markdownPath = "";
    if (root && path) {
      markdownPath = path.replace(root, "");
    }else{
      markdownPath = path;
    }

    // Implement your logic to check for markdown file
    // For example, you can use Tauri invoke to check for the file
    const hasMarkdown = await invoke("check_markdown", { root: root, path: markdownPath });
    return hasMarkdown;
 }

 const reloadMarkdown = async () => {
    // Implement your logic to reload the markdown file
    // For example, you can use Tauri invoke to trigger a reload
    await invoke("analyze_markdown", { path: path });
    console.log(`Markdown file ${path} analyzed`);
 };

 const getTotalCount = async (): Promise<number> => {
    let totalCount = 0;
    const total_count = await invoke<number>("get_total_count", { path: path});
    totalCount = total_count as number;
    return totalCount;
 };
  
 useEffect(() => {
    if (isFolder) {
      const fetchTotalCount = async () => {
        const totalCount = await getTotalCount() || 0;
        setCount(totalCount as number);
        setIsLoading(false);
      };
      
      setIsLoading(true);
      fetchTotalCount();
    }
 }, [isFolder, isOpen, name]);

 return (
    <tr>
      <td style={{ textAlign: "left" }}>
        {isFolder ? (
          <button className={""} onClick={onClick}>
            <AnalyzeButton className="small" disabled={isFolder} loading={isLoading} disabledText={true} onClick={handleAnalyzeClick} /> &nbsp;
            {name}
          </button>
        ) : (
          <button className={"small"} onClick={() => {}}>
            <AnalyzeButton className="small" disabled={isFolder} loading={isLoading} disabledText={true} onClick={handleAnalyzeClick} /> &nbsp;
            {name}
          </button>
        )}
      </td>
      <td>{type}</td>
      <td>{isFolder ? `${count} items` : ''}</td>
    </tr>
 );
};

export default ItemRow;

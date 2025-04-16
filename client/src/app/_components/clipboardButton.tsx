"use client";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { Check, Link2 } from "lucide-react";
import { useState } from "react";

interface Props {
  text: string;
}

function CopyToClipboardButton({ text }: Props) {
  const [isCopied, setIsCopied] = useState(false);

  const handleCopyClick = async () => {
    await navigator.clipboard.writeText(text);
    setIsCopied(true);
    setTimeout(() => setIsCopied(false), 2000);
  };

  return (
    <div>
      {!isCopied ? (
        <TooltipProvider delayDuration={0}>
          <Tooltip>
            <TooltipTrigger asChild>
              <Link2 onClick={handleCopyClick} className="h-4 w-4" />
            </TooltipTrigger>
            <TooltipContent>
              <p>Copy Link</p>
            </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      ) : (
        <Check className="h-4 w-4" />
      )}
    </div>
  );
}

export default CopyToClipboardButton;

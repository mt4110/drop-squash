import type { Metadata } from "next";
import { notFound } from "next/navigation";
import { buildMetadata, renderPage, resolvePage } from "../site-pages";

type PageProps = {
  params: Promise<{ slug?: string[] }>;
};

export async function generateMetadata({
  params,
}: PageProps): Promise<Metadata> {
  const path = normalize(await params);
  return resolvePage(path) ? buildMetadata(path) : {};
}

export default async function CatchAllPage({ params }: PageProps) {
  const path = normalize(await params);
  if (!resolvePage(path)) {
    notFound();
  }
  return renderPage(path);
}

function normalize(params: { slug?: string[] }) {
  return (params.slug ?? []).join("/");
}

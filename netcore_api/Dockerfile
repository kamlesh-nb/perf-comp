FROM mcr.microsoft.com/dotnet/aspnet:6.0 AS base
WORKDIR /app
EXPOSE 5041

ENV ASPNETCORE_URLS=http://+:5041

# Creates a non-root user with an explicit UID and adds permission to access the /app folder
# For more info, please refer to https://aka.ms/vscode-docker-dotnet-configure-containers
RUN adduser -u 5678 --disabled-password --gecos "" appuser && chown -R appuser /app
USER appuser

FROM mcr.microsoft.com/dotnet/sdk:6.0 AS build
WORKDIR /src
COPY ["netcore_api/netcore_api.csproj", "netcore_api/"]
RUN dotnet restore "netcore_api/netcore_api.csproj"
COPY . .
WORKDIR "/src/netcore_api"
RUN dotnet build "netcore_api.csproj" -c Release -o /app/build

FROM build AS publish
RUN dotnet publish "netcore_api.csproj" -c Release -o /app/publish /p:UseAppHost=false

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "netcore_api.dll"]
